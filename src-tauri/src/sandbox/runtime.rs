use rquickjs::{Context, Function, Runtime, Value as JSValue};
use std::collections::HashMap;

use crate::nodes::types::NodeValue;

/// Create a new JavaScript runtime context
pub fn create_runtime() -> Result<Runtime, String> {
    Runtime::new().map_err(|e| format!("Failed to create JS runtime: {}", e))
}

/// Set up the execution scope with variables
pub fn setup_scope(
    context: &Context,
    variables: HashMap<String, NodeValue>,
) -> Result<(), String> {
    for (name, value) in variables {
        let js_value = node_value_to_js(context, &value)?;
        context
            .globals()
            .set(&name, js_value)
            .map_err(|e| format!("Failed to set variable '{}': {}", name, e))?;
    }
    Ok(())
}

/// Convert NodeValue to JavaScript value
pub fn node_value_to_js<'js>(
    context: &Context<'js>,
    value: &NodeValue,
) -> Result<JSValue<'js>, String> {
    match value {
        NodeValue::Null => Ok(JSValue::new_null(context.clone())),
        NodeValue::Boolean(b) => Ok(JSValue::new_bool(context.clone(), *b)),
        NodeValue::Number(n) => Ok(JSValue::new_number(context.clone(), *n)),
        NodeValue::String(s) => JSValue::new_string(context.clone(), s)
            .map_err(|e| format!("Failed to create JS string: {}", e)),
        NodeValue::Array(arr) => {
            let js_arr = context
                .eval::<JSValue, _>("[]")
                .map_err(|e| format!("Failed to create JS array: {}", e))?;

            for (i, item) in arr.iter().enumerate() {
                let js_item = node_value_to_js(context, item)?;
                js_arr
                    .set(i as u32, js_item)
                    .map_err(|e| format!("Failed to set array item: {}", e))?;
            }
            Ok(js_arr)
        }
        NodeValue::Object(obj) => {
            // Convert serde_json::Value to JS object
            let json_str = serde_json::to_string(obj)
                .map_err(|e| format!("Failed to serialize object: {}", e))?;
            let js_obj = context
                .eval::<JSValue, _>(format!("({})", json_str))
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;
            Ok(js_obj)
        }
        NodeValue::File(file_ref) => {
            // Create a JS object with file reference info
            let js_obj = context
                .eval::<JSValue, _>("({})")
                .map_err(|e| format!("Failed to create file object: {}", e))?;
            js_obj
                .set(
                    "path",
                    JSValue::new_string(context.clone(), &file_ref.path)
                        .map_err(|e| format!("Failed to set path: {}", e))?,
                )
                .map_err(|e| format!("Failed to set path property: {}", e))?;
            js_obj
                .set("size", JSValue::new_number(context.clone(), file_ref.size as f64))
                .map_err(|e| format!("Failed to set size property: {}", e))?;
            Ok(js_obj)
        }
    }
}

/// Convert JavaScript value to NodeValue
pub fn js_value_to_node_value(context: &Context, value: JSValue) -> Result<NodeValue, String> {
    if value.is_null() || value.is_undefined() {
        return Ok(NodeValue::Null);
    }

    if value.is_bool() {
        return Ok(NodeValue::Boolean(
            value
                .as_bool()
                .ok_or("Failed to convert to boolean")?,
        ));
    }

    if value.is_number() {
        return Ok(NodeValue::Number(
            value.as_number().ok_or("Failed to convert to number")?,
        ));
    }

    if value.is_string() {
        let s = value
            .as_string()
            .ok_or("Failed to get string reference")?
            .to_string()
            .map_err(|e| format!("Failed to convert string: {}", e))?;
        return Ok(NodeValue::String(s));
    }

    if value.is_array() {
        let mut result = Vec::new();
        let len = value
            .get::<_, u32>("length")
            .map_err(|e| format!("Failed to get array length: {}", e))?;

        for i in 0..len {
            let item = value
                .get::<u32, JSValue>(i)
                .map_err(|e| format!("Failed to get array item {}: {}", i, e))?;
            result.push(js_value_to_node_value(context, item)?);
        }
        return Ok(NodeValue::Array(result));
    }

    if value.is_object() {
        // Convert to JSON string then parse
        let json_fn: Function = context
            .globals()
            .get("JSON")
            .map_err(|e| format!("Failed to get JSON object: {}", e))?
            .get("stringify")
            .map_err(|e| format!("Failed to get JSON.stringify: {}", e))?;

        let json_str: String = json_fn
            .call((value,))
            .map_err(|e| format!("Failed to stringify: {}", e))?;

        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        return Ok(NodeValue::Object(json_value));
    }

    Err(format!("Unsupported JavaScript type: {:?}", value.type_of()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_conversion() {
        let runtime = create_runtime().unwrap();
        let context = runtime.context();

        let node_val = NodeValue::Number(42.0);
        let js_val = node_value_to_js(&context, &node_val).unwrap();
        let back = js_value_to_node_value(&context, js_val).unwrap();

        match back {
            NodeValue::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_string_conversion() {
        let runtime = create_runtime().unwrap();
        let context = runtime.context();

        let node_val = NodeValue::String("hello".to_string());
        let js_val = node_value_to_js(&context, &node_val).unwrap();
        let back = js_value_to_node_value(&context, js_val).unwrap();

        match back {
            NodeValue::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_array_conversion() {
        let runtime = create_runtime().unwrap();
        let context = runtime.context();

        let node_val = NodeValue::Array(vec![
            NodeValue::Number(1.0),
            NodeValue::Number(2.0),
            NodeValue::Number(3.0),
        ]);
        let js_val = node_value_to_js(&context, &node_val).unwrap();
        let back = js_value_to_node_value(&context, js_val).unwrap();

        match back {
            NodeValue::Array(arr) => assert_eq!(arr.len(), 3),
            _ => panic!("Expected array"),
        }
    }
}
