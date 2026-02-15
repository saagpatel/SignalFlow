import type { PortDefinition } from "./portTypes";

export interface NodeCategory {
  id: string;
  label: string;
}

export interface ConfigFieldSchema {
  key: string;
  label: string;
  required?: boolean;
  widget:
    | "text"
    | "textarea"
    | "number"
    | "slider"
    | "checkbox"
    | "select"
    | "file-path-open"
    | "file-path-save"
    | "key-value"
    | "model-select";
  options?: { label: string; value: string }[];
  min?: number;
  max?: number;
  step?: number;
  placeholder?: string;
  rows?: number;
  monospace?: boolean;
}

export interface NodeDefinitionMeta {
  type: string;
  label: string;
  category: string;
  description: string;
  inputs: PortDefinition[];
  outputs: PortDefinition[];
  defaultConfig: Record<string, unknown>;
  configSchema?: ConfigFieldSchema[];
}

export const NODE_CATEGORIES: NodeCategory[] = [
  { id: "input", label: "Input" },
  { id: "transform", label: "Transform" },
  { id: "output", label: "Output" },
  { id: "control", label: "Control" },
  { id: "ai", label: "AI" },
  { id: "code", label: "Code" },
];

export const NODE_DEFINITIONS: NodeDefinitionMeta[] = [
  {
    type: "textInput",
    label: "Text Input",
    category: "input",
    description: "Output a static text value",
    inputs: [],
    outputs: [{ id: "value", label: "Value", type: "string", required: false }],
    defaultConfig: { value: "" },
    configSchema: [
      { key: "value", label: "Value", widget: "textarea", rows: 2, placeholder: "Enter text..." },
    ],
  },
  {
    type: "numberInput",
    label: "Number Input",
    category: "input",
    description: "Output a static number value",
    inputs: [],
    outputs: [{ id: "value", label: "Value", type: "number", required: false }],
    defaultConfig: { value: 0 },
    configSchema: [
      { key: "value", label: "Value", widget: "number" },
    ],
  },
  {
    type: "debug",
    label: "Debug",
    category: "output",
    description: "Display incoming data for inspection",
    inputs: [{ id: "input", label: "Input", type: "any", required: true }],
    outputs: [],
    defaultConfig: {},
    configSchema: [],
  },
  {
    type: "textTemplate",
    label: "Text Template",
    category: "transform",
    description: "Interpolate variables into a template string",
    inputs: [
      { id: "template", label: "Template", type: "string", required: true },
      { id: "variables", label: "Variables", type: "object", required: false },
    ],
    outputs: [{ id: "result", label: "Result", type: "string", required: false }],
    defaultConfig: { template: "Hello, {{name}}!" },
    configSchema: [
      { key: "template", label: "Template", required: true, widget: "textarea", rows: 4, placeholder: "Hello, {{name}}!" },
    ],
  },
  {
    type: "fileRead",
    label: "File Read",
    category: "input",
    description: "Read contents from a file",
    inputs: [{ id: "path", label: "Path", type: "string", required: false }],
    outputs: [
      { id: "content", label: "Content", type: "string", required: false },
      { id: "file", label: "File", type: "file", required: false },
    ],
    defaultConfig: { path: "" },
    configSchema: [
      { key: "path", label: "File Path", required: true, widget: "file-path-open", placeholder: "/path/to/file" },
    ],
  },
  {
    type: "fileWrite",
    label: "File Write",
    category: "output",
    description: "Write content to a file",
    inputs: [
      { id: "path", label: "Path", type: "string", required: true },
      { id: "content", label: "Content", type: "string", required: true },
    ],
    outputs: [{ id: "file", label: "File", type: "file", required: false }],
    defaultConfig: { path: "", append: false },
    configSchema: [
      { key: "path", label: "File Path", required: true, widget: "file-path-save", placeholder: "/path/to/file" },
      { key: "append", label: "Append to file", widget: "checkbox" },
    ],
  },
  {
    type: "httpRequest",
    label: "HTTP Request",
    category: "input",
    description: "Make an HTTP request",
    inputs: [
      { id: "url", label: "URL", type: "string", required: false },
      { id: "body", label: "Body", type: "string", required: false },
    ],
    outputs: [
      { id: "response", label: "Response", type: "string", required: false },
      { id: "status", label: "Status", type: "number", required: false },
    ],
    defaultConfig: { url: "", method: "GET", headers: "{}" },
    configSchema: [
      { key: "url", label: "URL", required: true, widget: "text", placeholder: "https://api.example.com" },
      {
        key: "method",
        label: "Method",
        widget: "select",
        options: [
          { label: "GET", value: "GET" },
          { label: "POST", value: "POST" },
          { label: "PUT", value: "PUT" },
          { label: "DELETE", value: "DELETE" },
          { label: "PATCH", value: "PATCH" },
        ],
      },
      { key: "headers", label: "Headers", widget: "key-value" },
    ],
  },
  {
    type: "jsonParse",
    label: "JSON Parse",
    category: "transform",
    description: "Parse a JSON string into an object",
    inputs: [{ id: "input", label: "Input", type: "string", required: true }],
    outputs: [{ id: "output", label: "Output", type: "object", required: false }],
    defaultConfig: {},
    configSchema: [],
  },
  {
    type: "regex",
    label: "Regex",
    category: "transform",
    description: "Match or replace using regular expressions",
    inputs: [{ id: "input", label: "Input", type: "string", required: true }],
    outputs: [
      { id: "matches", label: "Matches", type: "array", required: false },
      { id: "result", label: "Result", type: "string", required: false },
    ],
    defaultConfig: { pattern: "", flags: "g", mode: "match" },
    configSchema: [
      { key: "pattern", label: "Pattern", required: true, widget: "text", placeholder: "\\w+" },
      { key: "flags", label: "Flags", widget: "text", placeholder: "g, i, m, s" },
      {
        key: "mode",
        label: "Mode",
        widget: "select",
        options: [
          { label: "Match", value: "match" },
          { label: "Replace", value: "replace" },
        ],
      },
    ],
  },
  {
    type: "filter",
    label: "Filter",
    category: "transform",
    description: "Filter array elements by condition",
    inputs: [{ id: "input", label: "Input", type: "array", required: true }],
    outputs: [{ id: "output", label: "Output", type: "array", required: false }],
    defaultConfig: { condition: "item !== null", field: "" },
    configSchema: [
      { key: "condition", label: "Condition", required: true, widget: "text", placeholder: "item !== null" },
      { key: "field", label: "Field", widget: "text", placeholder: "Optional field name" },
    ],
  },
  {
    type: "map",
    label: "Map",
    category: "transform",
    description: "Transform each element in an array",
    inputs: [{ id: "input", label: "Input", type: "array", required: true }],
    outputs: [{ id: "output", label: "Output", type: "array", required: false }],
    defaultConfig: { expression: "item" },
    configSchema: [
      { key: "expression", label: "Expression", required: true, widget: "text", placeholder: "item" },
    ],
  },
  {
    type: "merge",
    label: "Merge",
    category: "transform",
    description: "Merge multiple inputs into one output",
    inputs: [
      { id: "a", label: "A", type: "any", required: true },
      { id: "b", label: "B", type: "any", required: true },
    ],
    outputs: [{ id: "output", label: "Output", type: "array", required: false }],
    defaultConfig: {},
    configSchema: [],
  },
  {
    type: "split",
    label: "Split",
    category: "transform",
    description: "Split a string or array into parts",
    inputs: [{ id: "input", label: "Input", type: "string", required: true }],
    outputs: [{ id: "output", label: "Output", type: "array", required: false }],
    defaultConfig: { delimiter: "," },
    configSchema: [
      {
        key: "delimiter",
        label: "Delimiter",
        widget: "select",
        options: [
          { label: "Comma (,)", value: "," },
          { label: "Newline (\\n)", value: "\n" },
          { label: "Tab (\\t)", value: "\t" },
          { label: "Pipe (|)", value: "|" },
          { label: "Space", value: " " },
        ],
      },
    ],
  },
  {
    type: "conditional",
    label: "Conditional",
    category: "control",
    description: "Route data based on a condition",
    inputs: [
      { id: "input", label: "Input", type: "any", required: true },
      { id: "condition", label: "Condition", type: "boolean", required: false },
    ],
    outputs: [
      { id: "true", label: "True", type: "any", required: false },
      { id: "false", label: "False", type: "any", required: false },
    ],
    defaultConfig: { expression: "input !== null" },
    configSchema: [
      { key: "expression", label: "Expression", required: true, widget: "text", placeholder: "input !== null" },
    ],
  },
  {
    type: "llmPrompt",
    label: "LLM Prompt",
    category: "ai",
    description: "Generate text using a local LLM via Ollama",
    inputs: [{ id: "prompt", label: "Prompt", type: "string", required: true }],
    outputs: [{ id: "response", label: "Response", type: "string", required: false }],
    defaultConfig: { model: "llama3.2", temperature: 0.7, systemPrompt: "" },
    configSchema: [
      { key: "model", label: "Model", widget: "model-select" },
      { key: "temperature", label: "Temperature", widget: "slider", min: 0, max: 2, step: 0.1 },
      { key: "systemPrompt", label: "System Prompt", widget: "textarea", rows: 4, placeholder: "You are a helpful assistant..." },
    ],
  },
  {
    type: "llmChat",
    label: "LLM Chat",
    category: "ai",
    description: "Multi-turn chat with a local LLM",
    inputs: [
      { id: "message", label: "Message", type: "string", required: true },
      { id: "history", label: "History", type: "array", required: false },
    ],
    outputs: [
      { id: "response", label: "Response", type: "string", required: false },
      { id: "history", label: "History", type: "array", required: false },
    ],
    defaultConfig: { model: "llama3.2", temperature: 0.7, systemPrompt: "" },
    configSchema: [
      { key: "model", label: "Model", widget: "model-select" },
      { key: "temperature", label: "Temperature", widget: "slider", min: 0, max: 2, step: 0.1 },
      { key: "systemPrompt", label: "System Prompt", widget: "textarea", rows: 4, placeholder: "You are a helpful assistant..." },
    ],
  },
  {
    type: "code",
    label: "Code",
    category: "code",
    description: "Run custom JavaScript code",
    inputs: [{ id: "input", label: "Input", type: "any", required: false }],
    outputs: [{ id: "output", label: "Output", type: "any", required: false }],
    defaultConfig: { code: "return input;" },
    configSchema: [
      { key: "code", label: "Code", required: true, widget: "textarea", rows: 6, monospace: true, placeholder: "return input;" },
    ],
  },
  {
    type: "tryCatch",
    label: "Try/Catch",
    category: "control",
    description: "Handle errors from upstream nodes gracefully",
    inputs: [{ id: "input", label: "Input", type: "any", required: true }],
    outputs: [
      { id: "success", label: "Success", type: "any", required: false },
      { id: "error", label: "Error", type: "string", required: false },
    ],
    defaultConfig: {},
    configSchema: [],
  },
  {
    type: "forEach",
    label: "For Each",
    category: "control",
    description: "Iterate over array items and execute for each",
    inputs: [{ id: "array", label: "Array", type: "array", required: true }],
    outputs: [{ id: "results", label: "Results", type: "array", required: false }],
    defaultConfig: {},
    configSchema: [],
  },
];

export function getNodeDefinition(type: string): NodeDefinitionMeta | undefined {
  return NODE_DEFINITIONS.find((d) => d.type === type);
}

export function getNodesByCategory(category: string): NodeDefinitionMeta[] {
  return NODE_DEFINITIONS.filter((d) => d.category === category);
}
