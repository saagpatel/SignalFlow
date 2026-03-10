import { copyFileSync, mkdirSync } from "node:fs";

mkdirSync("src-tauri/target/release/bundle/release-notes", { recursive: true });
copyFileSync(
  "docs/release-notes-v1.0.0.md",
  "src-tauri/target/release/bundle/release-notes/RELEASE_NOTES.md",
);

console.log(
  "wrote src-tauri/target/release/bundle/release-notes/RELEASE_NOTES.md",
);
