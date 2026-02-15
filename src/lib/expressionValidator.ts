/**
 * Validate JavaScript expression syntax at compile time
 * @param code JavaScript code string
 * @returns { valid: boolean; error?: string }
 */
export function validateExpression(code: string): { valid: boolean; error?: string } {
  if (!code || code.trim().length === 0) {
    return { valid: false, error: "Expression cannot be empty" };
  }

  try {
    // Try to parse as function body
    // This will catch syntax errors without executing the code
    new Function(`return (${code})`);
    return { valid: true };
  } catch (e) {
    return {
      valid: false,
      error: e instanceof Error ? e.message : String(e),
    };
  }
}

/**
 * Validate that expression has access to correct scope
 * Expected variables: input, item, index, etc.
 */
export function validateExpressionScope(
  code: string,
  allowedVars: string[]
): { valid: boolean; error?: string } {
  // Built-in JavaScript globals and Math functions to allow
  const builtins = new Set([
    "Math",
    "JSON",
    "Array",
    "Object",
    "String",
    "Number",
    "Boolean",
    "Date",
    "console",
    "true",
    "false",
    "null",
    "undefined",
    "NaN",
    "Infinity",
    "isNaN",
    "isFinite",
    "parseInt",
    "parseFloat",
    // Math methods
    "abs",
    "ceil",
    "floor",
    "round",
    "max",
    "min",
    "pow",
    "sqrt",
    "random",
  ]);

  // Extract identifier usage (simple regex, not perfect but good enough)
  const identifierRegex = /\b([a-zA-Z_$][a-zA-Z0-9_$]*)\b/g;
  const usedVars = new Set<string>();

  let match;
  while ((match = identifierRegex.exec(code)) !== null) {
    usedVars.add(match[1]);
  }

  const allowedSet = new Set([...allowedVars, ...builtins]);

  // Check for undefined variables
  for (const variable of usedVars) {
    if (!allowedSet.has(variable)) {
      return {
        valid: false,
        error: `Undefined variable: '${variable}'. Available: ${allowedVars.join(", ")}`,
      };
    }
  }

  return { valid: true };
}

/**
 * Get suggested variables for autocomplete based on node type
 */
export function getSuggestedVariables(nodeType: string): string[] {
  switch (nodeType) {
    case "filter":
    case "map":
      return ["item", "index"];
    case "conditional":
    case "code":
      return ["input"];
    default:
      return [];
  }
}

/**
 * Validate expression for a specific node type
 */
export function validateExpressionForNode(
  nodeType: string,
  expression: string
): { valid: boolean; error?: string } {
  // First check syntax
  const syntaxCheck = validateExpression(expression);
  if (!syntaxCheck.valid) {
    return syntaxCheck;
  }

  // Then check scope
  const allowedVars = getSuggestedVariables(nodeType);
  if (allowedVars.length > 0) {
    return validateExpressionScope(expression, allowedVars);
  }

  return { valid: true };
}

/**
 * Format error message for display
 */
export function formatValidationError(error: string): string {
  // Clean up Function constructor errors
  if (error.includes("Unexpected token")) {
    return "Syntax error: Invalid JavaScript expression";
  }
  if (error.includes("Unexpected identifier")) {
    return "Syntax error: Unexpected identifier in expression";
  }
  if (error.includes("Unexpected end of input")) {
    return "Syntax error: Incomplete expression";
  }
  return error;
}
