import { existsSync, readFileSync } from "node:fs";

const requiredFiles = [
  "README.md",
  "docs/launch-contract.md",
  "docs/release-readiness.md",
  "docs/release-notes-v1.0.0.md",
  "docs/adr/0000-template.md",
  "openapi/openapi.generated.json",
];

const missing = requiredFiles.filter((file) => !existsSync(file));

if (missing.length > 0) {
  console.error("Missing required documentation artifacts:");
  for (const file of missing) {
    console.error(`- ${file}`);
  }
  process.exit(1);
}

try {
  JSON.parse(readFileSync("openapi/openapi.generated.json", "utf8"));
} catch (error) {
  console.error("openapi/openapi.generated.json must be valid JSON.");
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}

const readme = readFileSync("README.md", "utf8");
for (const section of ["Launch Contract", "Release Readiness"]) {
  if (!readme.includes(section)) {
    console.error(`README.md must reference "${section}".`);
    process.exit(1);
  }
}

console.log("Documentation artifact checks passed.");
