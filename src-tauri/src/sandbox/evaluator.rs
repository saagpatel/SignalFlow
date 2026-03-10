use std::collections::HashMap;

use rquickjs::{Context, Runtime, Value as JsValue};

use crate::types::NodeValue;

/// Evaluate a JavaScript expression with a single 'input' variable
pub fn evaluate_expression(code: &str, input: &NodeValue) -> Result<NodeValue, String> {
    let mut vars = HashMap::new();
    vars.insert("input".to_string(), input.clone());
    evaluate_expression_with_scope(code, vars)
}

/// Evaluate a JavaScript expression with custom scope variables
pub fn evaluate_expression_with_scope(
    code: &str,
    variables: HashMap<String, NodeValue>,
) -> Result<NodeValue, String> {
    if code.trim().is_empty() {
        return Err("Expression is empty".to_string());
    }

    let runtime = Runtime::new().map_err(|e| format!("Failed to create JS runtime: {}", e))?;
    let context =
        Context::full(&runtime).map_err(|e| format!("Failed to create JS context: {}", e))?;
    let wrapped_code = wrap_code_for_execution(code);

    context.with(|ctx| {
        inject_scope(&ctx, variables)?;
        let result: JsValue = ctx
            .eval(wrapped_code)
            .map_err(|e| format!("JavaScript execution error: {}", e))?;
        js_to_node_value(&ctx, result)
    })
}

fn wrap_code_for_execution(code: &str) -> String {
    let trimmed = code.trim();
    let body = if looks_like_expression(trimmed) {
        format!("return ({trimmed});")
    } else {
        trimmed.to_string()
    };
    format!("(() => {{ {body} }})()")
}

fn looks_like_expression(code: &str) -> bool {
    !code.contains(';') && !code.contains('\n') && !code.trim_start().starts_with("return")
}

fn inject_scope(
    ctx: &rquickjs::Ctx<'_>,
    variables: HashMap<String, NodeValue>,
) -> Result<(), String> {
    let scope: serde_json::Map<String, serde_json::Value> = variables
        .into_iter()
        .map(|(key, value)| (key, value.to_json_value()))
        .collect();
    let scope_json =
        serde_json::to_string(&scope).map_err(|e| format!("Failed to serialize scope: {}", e))?;
    let setup_script = format!("Object.assign(globalThis, {scope_json});");
    ctx.eval::<(), _>(setup_script)
        .map_err(|e| format!("Failed to inject scope variables: {}", e))
}

fn js_to_node_value<'js>(
    ctx: &rquickjs::Ctx<'js>,
    value: JsValue<'js>,
) -> Result<NodeValue, String> {
    let Some(json_str) = ctx
        .json_stringify(value)
        .map_err(|e| format!("Failed to stringify JS value: {}", e))?
    else {
        return Ok(NodeValue::Null);
    };

    let json_text = json_str
        .to_string()
        .map_err(|e| format!("Failed to convert JS string: {}", e))?;
    let parsed: serde_json::Value = serde_json::from_str(&json_text)
        .map_err(|e| format!("Failed to parse JS result JSON: {}", e))?;

    Ok(json_to_node_value(parsed))
}

fn json_to_node_value(value: serde_json::Value) -> NodeValue {
    match value {
        serde_json::Value::Null => NodeValue::Null,
        serde_json::Value::Bool(v) => NodeValue::Boolean(v),
        serde_json::Value::Number(v) => NodeValue::Number(v.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(v) => NodeValue::String(v),
        serde_json::Value::Array(values) => {
            NodeValue::Array(values.into_iter().map(json_to_node_value).collect())
        }
        serde_json::Value::Object(mut obj) => {
            if obj.len() == 1 {
                if let Some(path_value) = obj.remove("path") {
                    if let serde_json::Value::String(path) = path_value {
                        return NodeValue::File { path };
                    }
                    obj.insert("path".to_string(), path_value);
                }
            }
            NodeValue::Object(obj.into_iter().collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_simple_expression() {
        let input = NodeValue::Number(5.0);
        let result = evaluate_expression("return input * 2", &input).unwrap();

        match result {
            NodeValue::Number(n) => assert_eq!(n, 10.0),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_evaluate_string_concat() {
        let input = NodeValue::String("hello".to_string());
        let result = evaluate_expression("return input + ' world'", &input).unwrap();

        match result {
            NodeValue::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected string result"),
        }
    }

    #[test]
    fn test_evaluate_with_custom_scope() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), NodeValue::Number(3.0));
        vars.insert("y".to_string(), NodeValue::Number(4.0));

        let result = evaluate_expression_with_scope("return x + y", vars).unwrap();

        match result {
            NodeValue::Number(n) => assert_eq!(n, 7.0),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_evaluate_array_map() {
        let input = NodeValue::Array(vec![
            NodeValue::Number(1.0),
            NodeValue::Number(2.0),
            NodeValue::Number(3.0),
        ]);

        let code = "return input.map(x => x * 2)";
        let result = evaluate_expression(code, &input).unwrap();

        match result {
            NodeValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                match &arr[0] {
                    NodeValue::Number(n) => assert_eq!(*n, 2.0),
                    _ => panic!("Expected number"),
                }
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_evaluate_expression_without_return_keyword() {
        let input = NodeValue::Number(5.0);
        let result = evaluate_expression("input * 3", &input).unwrap();

        match result {
            NodeValue::Number(n) => assert_eq!(n, 15.0),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_evaluate_scope_variables_without_return_keyword() {
        let mut vars = HashMap::new();
        vars.insert("item".to_string(), NodeValue::Null);
        let result = evaluate_expression_with_scope("item !== null", vars).unwrap();

        match result {
            NodeValue::Boolean(v) => assert!(!v),
            _ => panic!("Expected boolean result"),
        }
    }

    #[test]
    fn test_evaluate_invalid_syntax() {
        let input = NodeValue::Number(5.0);
        let result = evaluate_expression("return input +", &input);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("JavaScript execution error"));
    }

    #[test]
    fn test_evaluate_undefined_variable() {
        let input = NodeValue::Number(5.0);
        let result = evaluate_expression("return undefined_var * 2", &input);

        assert!(result.is_err());
    }
}
