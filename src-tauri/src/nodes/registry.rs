use std::collections::HashMap;

use super::NodeExecutor;
use super::input::*;
use super::transform::*;
use super::output::*;
use super::control::*;
use super::ai::*;

pub struct NodeRegistry {
    executors: HashMap<String, Box<dyn NodeExecutor>>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        let mut executors: HashMap<String, Box<dyn NodeExecutor>> = HashMap::new();

        // Input nodes
        executors.insert("textInput".to_string(), Box::new(TextInputExecutor));
        executors.insert("numberInput".to_string(), Box::new(NumberInputExecutor));
        executors.insert("fileRead".to_string(), Box::new(FileReadExecutor));
        executors.insert("httpRequest".to_string(), Box::new(HttpRequestExecutor));

        // Transform nodes
        executors.insert("textTemplate".to_string(), Box::new(TextTemplateExecutor));
        executors.insert("jsonParse".to_string(), Box::new(JsonParseExecutor));
        executors.insert("regex".to_string(), Box::new(RegexExecutor));
        executors.insert("filter".to_string(), Box::new(FilterExecutor));
        executors.insert("map".to_string(), Box::new(MapExecutor));
        executors.insert("merge".to_string(), Box::new(MergeExecutor));
        executors.insert("split".to_string(), Box::new(SplitExecutor));

        // Output nodes
        executors.insert("debug".to_string(), Box::new(DebugExecutor));
        executors.insert("fileWrite".to_string(), Box::new(FileWriteExecutor));

        // Control nodes
        executors.insert("conditional".to_string(), Box::new(ConditionalExecutor));
        executors.insert("code".to_string(), Box::new(CodeExecutor));
        executors.insert("tryCatch".to_string(), Box::new(TryCatchExecutor));
        executors.insert("forEach".to_string(), Box::new(ForEachExecutor));

        // AI nodes
        executors.insert("llmPrompt".to_string(), Box::new(LlmPromptExecutor));
        executors.insert("llmChat".to_string(), Box::new(LlmChatExecutor));

        Self { executors }
    }

    pub fn get(&self, node_type: &str) -> Option<&dyn NodeExecutor> {
        self.executors.get(node_type).map(|e| e.as_ref())
    }

    pub fn has(&self, node_type: &str) -> bool {
        self.executors.contains_key(node_type)
    }

    pub fn register(&mut self, executor: Box<dyn NodeExecutor>) {
        executors_insert(&mut self.executors, executor);
    }
}

fn executors_insert(
    map: &mut HashMap<String, Box<dyn NodeExecutor>>,
    executor: Box<dyn NodeExecutor>,
) {
    map.insert(executor.node_type().to_string(), executor);
}
