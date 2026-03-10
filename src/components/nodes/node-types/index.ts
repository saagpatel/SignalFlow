import type { NodeTypes } from "@xyflow/react";
import { TextInputNode } from "./TextInputNode";
import { NumberInputNode } from "./NumberInputNode";
import { DebugNode } from "./DebugNode";
import { TextTemplateNode } from "./TextTemplateNode";
import { FileReadNode } from "./FileReadNode";
import { FileWriteNode } from "./FileWriteNode";
import { HttpRequestNode } from "./HttpRequestNode";
import { LlmPromptNode } from "./LlmPromptNode";
import { ConditionalNode } from "./ConditionalNode";
import { CodeNode } from "./CodeNode";
import { RegexNode } from "./RegexNode";
import { GenericNode } from "./GenericNode";

export const nodeTypes: NodeTypes = {
  textInput: TextInputNode,
  numberInput: NumberInputNode,
  debug: DebugNode,
  textTemplate: TextTemplateNode,
  fileRead: FileReadNode,
  fileWrite: FileWriteNode,
  httpRequest: HttpRequestNode,
  regex: RegexNode,
  conditional: ConditionalNode,
  llmPrompt: LlmPromptNode,
  llmChat: LlmPromptNode, // Same display as LlmPrompt
  code: CodeNode,
  // Remaining simple nodes use GenericNode
  jsonParse: GenericNode,
  filter: GenericNode,
  map: GenericNode,
  merge: GenericNode,
  split: GenericNode,
  tryCatch: GenericNode,
  forEach: GenericNode,
};
