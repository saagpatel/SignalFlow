# SignalFlow v1.0.0 Release Notes

## What shipped

- Desktop workflow editor for building local data and AI pipelines visually
- SQLite-backed flow persistence with recent flows, save/open/delete, and restore-last-flow behavior
- Local Rust execution engine with progress events, cancellation, output previews, and validation feedback
- Shared node catalog across frontend and backend surfaces
- Ollama-backed AI nodes with configurable endpoint support
- macOS-first release workflow with manual GitHub Release distribution

## Verification summary

- Canonical local gate: `pnpm verify`
- Unsigned bundle rehearsal: `pnpm tauri build`
- Checksum generation: `pnpm release:checksum`

## Known limitations

- GitHub Release publication is still manual for `v1.0.0`
- Apple signing and notarization are required before public signed macOS distribution
- Live Ollama integration still depends on a reachable local Ollama endpoint and installed model

## Upgrade / rollback note

- Back up the local `signalflow.db` before final release-candidate testing
- Keep the last known-good `.dmg` available while validating the `v1.0.0` build
