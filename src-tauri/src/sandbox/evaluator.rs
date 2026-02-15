use std::collections::HashMap;

use crate::nodes::types::NodeValue;

use super::runtime::{create_runtime, js_value_to_node_value, node_value_to_js, setup_scope};

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
    // Create runtime and context
    let runtime = create_runtime()?;
    let context = runtime.context();

    // Set up scope with variables
    setup_scope(&context, variables)?;

    // Wrap code to ensure it returns a value
    let wrapped_code = if code.trim().starts_with("return") {
        code.to_string()
    } else {
        format!("(function() {{ {} }})()", code)
    };

    // Execute code
    let result = context
        .eval::<rquickjs::Value, _>(&wrapped_code)
        .map_err(|e| format!("JavaScript execution error: {}", e))?;

    // Convert result back to NodeValue
    js_value_to_node_value(&context, result)
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
