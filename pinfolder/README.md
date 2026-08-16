# Pinfolder

A local-first moodboard & planning app, inspired by Milanote. Every board is
plain files — `board.json` plus an `assets/` folder — inside a folder you
pick on your own computer. Nested boards are literally nested folders. No
account, no server, no database engine.

Built with [Tauri](https://tauri.app) (Rust) + React + TypeScript. See the
full architecture and feature spec this is being built from for the reasoning
behind these choices.

## Status: Phase 0 (shell)

What works today:

- Pick a root folder (native OS dialog) — remembered for next launch
- Create a project (a folder with `board.json` + a stable `.board-id`)
- List projects in the chosen root, open one
- Board view reads/writes `board.json` on disk (atomic writes)

Canvas tools (notes, images, connectors, nested boards, …) land in later
phases.

## Prerequisites

- [Bun](https://bun.sh)
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Linux only: GTK/WebKitGTK dev packages —
  `libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev patchelf build-essential libssl-dev libayatana-appindicator3-dev`
  (see [Tauri's prerequisites guide](https://tauri.app/start/prerequisites/)
  for macOS/Windows)

## Develop

```bash
bun install
bun run tauri dev
```

## Build

```bash
bun run tauri build
```

## Project layout

```
src/            React UI (screens/, lib/tauri.ts — typed IPC bridge)
src-tauri/      Rust backend — the only code with filesystem access
  src/commands.rs   Tauri commands the UI calls (pick folder, read/write board, …)
  src/model.rs      Shared data types (ProjectSummary, BoardDocument)
  src/fsutil.rs     Atomic file writes
  src/settings.rs   Persisted app settings (last-used root folder)
```
