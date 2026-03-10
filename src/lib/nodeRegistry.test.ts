import { describe, expect, it } from "vitest";
import {
  NODE_CATEGORIES,
  getNodeDefinition,
  getNodesByCategory,
} from "./nodeRegistry";

describe("nodeRegistry", () => {
  it("returns the definition for a known node type", () => {
    const definition = getNodeDefinition("textInput");

    expect(definition).toMatchObject({
      type: "textInput",
      category: "input",
    });
  });

  it("returns all nodes in a given category", () => {
    const inputNodes = getNodesByCategory("input");

    expect(inputNodes.length).toBeGreaterThan(0);
    expect(
      inputNodes.every((definition) => definition.category === "input"),
    ).toBe(true);
  });

  it("exposes the shared node categories", () => {
    expect(NODE_CATEGORIES.map((category) => category.id)).toContain("ai");
  });
});
