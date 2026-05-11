# SignalFlow — Portfolio Disposition

**Status:** Release Frozen — v1.0.0 release-readiness scope complete
on `origin/main` with release-notes, launch contract, and readiness
checklist. Awaiting operator-only Apple signing. Joins the
signing-frozen cluster (now 10 repos).

> Disposition uses strict `origin/main` verification.

---

## Verification posture

This repo has both `origin` (`saagpatel/SignalFlow`) and
`legacy-origin` (`saagar210/SignalFlow`) remotes. Disposition reads
`origin/main` only.

Specifically verified:

- `origin/main` tip: `5efcfd9` chore: add pull request template
- Recent substantive commits on `origin/main`:
  - `abecdfa` feat: Add advanced features and release pipeline (Phase 7)
  - `ed83f70` feat: Add comprehensive testing and Settings UI (Phases 5-6)
  - `d1df6d4` feat: Implement Code node, expression evaluation, and CI/CD
  - `ebf3f63` feat: implement comprehensive UX overhaul (40% → 95% usability)
  - `f7c7351` feat: complete SignalFlow — visual dataflow programming desktop app
- `docs/release-readiness.md`, `docs/release-notes-v1.0.0.md`,
  `docs/launch-contract.md`, `docs/adr/` all on `origin/main`
- Standard portfolio scaffolding: `.codex/`, `.github/`, `.husky/`,
  `.perf-baselines/`, CHANGELOG, LICENSE, commitlint config,
  lighthouse config

---

## Current state in one paragraph

SignalFlow is a visual dataflow programming desktop app — drag nodes
onto a canvas, connect them with wires, run pipelines locally. Tauri

- TypeScript frontend + Rust backend, ReactFlow-powered graph
  editor, SQLite WAL persistence, Ollama integration for local LLM
  nodes. Node library covers file I/O, JSON parsing, HTTP requests,
  regex transforms, conditional routing, plus Ollama prompt/chat. Live
  execution animates data through the graph; pre-run validation
  flags misconfigurations. v1.0.0 release-readiness scope (Phase 7)
  complete; the UX overhaul commit explicitly took the app from
  "40% → 95% usability."

For full detail:

- `docs/release-readiness.md` — release gates
- `docs/release-notes-v1.0.0.md` — v1.0.0 release notes (drafted)
- `docs/launch-contract.md` — launch criteria
- `docs/adr/`
- `README.md`

---

## Portfolio operating system instructions

| Aspect               | Posture                                                                                                                                                                                              |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Portfolio status     | `Release Frozen`                                                                                                                                                                                     |
| Review cadence       | Suspend overdue counting                                                                                                                                                                             |
| Resurface conditions | (a) Apple signing credentials wired, (b) operator decides to ship v1.0.0 unsigned (not currently planned), or (c) operator opens a v1.1+ scope packet (new node types, cloud/team sync, marketplace) |
| Co-batch with        | Signing cluster: DesktopPEt / ContentEngine / AIGCCore / Relay / FreeLanceInvoice / Nexus / DeepTank / OPscinema / ShipKit / **SignalFlow** — **now 10 repos**.                                      |

---

## Why "Release Frozen" instead of other dispositions

- **Active** — wrong. v1.0.0 scope is complete; release notes are
  drafted.
- **Cold Storage / Archived** — wrong. UX overhaul + Phase 7 are
  recent substantive work, not idle.
- **Release Frozen** — correct, same shape as the rest of the
  signing cluster.

---

## Unblock trigger (operator)

When ready to ship v1.0.0:

1. Wire Apple Developer ID + notarization credentials.
2. Run release-readiness checklist (per `docs/release-readiness.md`).
3. Verify the signed/notarized installer opens cleanly with no
   Gatekeeper warnings.
4. Publish v1.0.0 with the pre-drafted release notes.

Estimated operator time once credentials are in hand: ~3 hours
including notarization round-trip.

---

## Reactivation procedure (for the next code session)

1. **Verify local clone tracking.** `git branch -vv` — if `main`
   tracks `legacy-origin/main`, retarget to `origin/main`. Trap
   reference: FreeLanceInvoice + PersonalKBDrafter corrections.
2. Delete stale `codex/*` branches (most are merged-history).
3. Re-run `pnpm install && pnpm verify` to confirm the toolchain
   still works after the freeze.
4. Re-run the release-readiness checklist before tagging v1.0.0.

---

## Last known reference

| Field                     | Value                                                                                                                 |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `origin/main` tip         | `5efcfd9` chore: add pull request template                                                                            |
| Last substantive commit   | `abecdfa` feat: Add advanced features and release pipeline (Phase 7)                                                  |
| Major version             | v1.0.0 (release notes drafted on origin/main)                                                                         |
| Build verification status | green                                                                                                                 |
| Blocker                   | Apple signing + notarization (operator-only)                                                                          |
| Migration note            | `legacy-origin` points at frozen `saagar210/SignalFlow`; do not push there                                            |
| Notable                   | UX overhaul commit `ebf3f63` explicitly tracks "40% → 95% usability" — strong signal of pre-release polish discipline |
