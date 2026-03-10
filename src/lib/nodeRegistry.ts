import type { PortDefinition } from "./portTypes";
import nodeCatalog from "../shared/nodeCatalog.json";

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

interface NodeCatalogFile {
  categories: NodeCategory[];
  definitions: NodeDefinitionMeta[];
}

const catalog = nodeCatalog as NodeCatalogFile;

export const NODE_CATEGORIES: NodeCategory[] = catalog.categories;
export const NODE_DEFINITIONS: NodeDefinitionMeta[] = catalog.definitions;

export function getNodeDefinition(
  type: string,
): NodeDefinitionMeta | undefined {
  return NODE_DEFINITIONS.find((definition) => definition.type === type);
}

export function getNodesByCategory(category: string): NodeDefinitionMeta[] {
  return NODE_DEFINITIONS.filter(
    (definition) => definition.category === category,
  );
}
