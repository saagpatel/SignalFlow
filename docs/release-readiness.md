# SignalFlow Release Readiness

## Current release posture

- Target: macOS-first `v1.0.0`
- Distribution: manual GitHub Releases
- Canonical local verification: `pnpm verify`
- Release workflow output: uploaded macOS `.dmg`, app archive, `SHA256SUMS.txt`, and release-notes artifact
- Current release metadata: version `1.0.0`, bundle identifier `com.signalflow.desktop`

## Required local checks before a release candidate

```bash
pnpm verify
pnpm tauri build
pnpm release:pack-macos-app
pnpm release:notes
pnpm release:checksum
pnpm release:validate-macos
pnpm perf:summary
```

## External blockers to signed distribution

- Apple signing and notarization are not fully automated in-repo yet.
- A signed public build still requires Apple Developer credentials and secure secret storage.
- Typical required inputs:
  - `APPLE_CERTIFICATE`
  - `APPLE_CERTIFICATE_PASSWORD`
  - `APPLE_SIGNING_IDENTITY`
  - `APPLE_ID`
  - `APPLE_PASSWORD`
  - `APPLE_TEAM_ID`

## Immediately after signing and notarization

Run the final signed-artifact validation before publishing:

```bash
pnpm release:validate-signed-macos
```

This checks Gatekeeper acceptance for the DMG and app bundle, then verifies the app’s code signing details after mounting the signed disk image.

## Release workflow expectations

- CI should be green before a release tag is created.
- Release workflow builds macOS artifacts from a frozen lockfile.
- GitHub Release publishing remains manual for `v1.0.0`.
- Uploaded workflow artifacts should be used to assemble the final release draft.
- The unsigned local bundle should produce `SignalFlow_1.0.0_aarch64.dmg`, `SignalFlow.app.tar.gz`, `SHA256SUMS.txt`, and `RELEASE_NOTES.md`.

## Recommended RC rehearsal

1. Run `pnpm verify`.
2. Build the desktop bundle with `pnpm tauri build`.
3. Package the app archive with `pnpm release:pack-macos-app`.
4. Generate release notes with `pnpm release:notes`.
5. Generate checksums with `pnpm release:checksum`.
6. Validate the `.dmg` and app archive with `pnpm release:validate-macos`.
7. Install the generated `.dmg`.
8. Open SignalFlow, load an existing flow, run it, save it, reopen it, and verify persistence.
9. If testing AI flows, confirm the configured Ollama endpoint is reachable and the selected model is installed.
