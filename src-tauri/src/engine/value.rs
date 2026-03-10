use crate::types::NodeValue;

impl NodeValue {
    pub fn coerce_to_string(&self) -> String {
        match self {
            NodeValue::String(s) => s.clone(),
            NodeValue::Number(n) => n.to_string(),
            NodeValue::Boolean(b) => b.to_string(),
            NodeValue::Null => String::new(),
            NodeValue::Array(arr) => {
                serde_json::to_string(&arr.iter().map(|v| v.to_json_value()).collect::<Vec<_>>())
                    .unwrap_or_default()
            }
            NodeValue::Object(obj) => serde_json::to_string(obj).unwrap_or_default(),
            NodeValue::File { path } => path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_coercion() {
        assert_eq!(NodeValue::Number(42.0).coerce_to_string(), "42");
        assert_eq!(NodeValue::Boolean(true).coerce_to_string(), "true");
        assert_eq!(
            NodeValue::String("hello".into()).coerce_to_string(),
            "hello"
        );
    }
}
