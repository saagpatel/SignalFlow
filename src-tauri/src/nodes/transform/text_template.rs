use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct TextTemplateExecutor;

#[async_trait]
impl NodeExecutor for TextTemplateExecutor {
    fn node_type(&self) -> &'static str {
        "textTemplate"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let template = inputs
            .get("template")
            .and_then(|v| v.as_string())
            .or_else(|| {
                config
                    .get("template")
                    .and_then(|v| v.as_str())
                    .map(String::from)
            })
            .unwrap_or_default();

        let mut result = template.clone();

        // Interpolate from variables input (object)
        if let Some(NodeValue::Object(vars)) = inputs.get("variables") {
            for (key, value) in vars {
                let placeholder = format!("{{{{{}}}}}", key);
                let replacement = match value {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                result = result.replace(&placeholder, &replacement);
            }
        }

        // Also interpolate from any other inputs
        for (key, value) in &inputs {
            if key == "template" || key == "variables" {
                continue;
            }
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, &value.coerce_to_string());
        }

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), NodeValue::String(result));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_template_interpolation() {
        let executor = TextTemplateExecutor;
        let config = serde_json::json!({ "template": "Hello, {{name}}!" });
        let mut inputs = HashMap::new();

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), serde_json::json!("World"));
        inputs.insert("variables".to_string(), NodeValue::Object(vars));

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await.unwrap();
        assert_eq!(
            result.get("result").unwrap().as_string().unwrap(),
            "Hello, World!"
        );
    }

    #[tokio::test]
    async fn test_template_from_input() {
        let executor = TextTemplateExecutor;
        let config = serde_json::json!({});
        let mut inputs = HashMap::new();
        inputs.insert(
            "template".to_string(),
            NodeValue::String("Value: {{x}}".to_string()),
        );
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), serde_json::json!(42));
        inputs.insert("variables".to_string(), NodeValue::Object(vars));

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await.unwrap();
        assert_eq!(
            result.get("result").unwrap().as_string().unwrap(),
            "Value: 42"
        );
    }
}
