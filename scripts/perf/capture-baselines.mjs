import { mkdirSync, readFileSync, writeFileSync } from "node:fs";

const baselineSources = [
  [".perf-results/bundle.json", ".perf-baselines/bundle.json"],
  [".perf-results/build-time.json", ".perf-baselines/build-time.json"],
];

mkdirSync(".perf-baselines", { recursive: true });

for (const [sourcePath, targetPath] of baselineSources) {
  const contents = JSON.parse(readFileSync(sourcePath, "utf8"));
  writeFileSync(targetPath, `${JSON.stringify(contents, null, 2)}\n`);
  console.log(`captured ${targetPath}`);
}
