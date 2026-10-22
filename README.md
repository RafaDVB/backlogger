# Backlogger

A local-first desktop app for tracking your video game backlog. Keep an owned/interested
library, fill a six-slot "Now Playing" shelf, and record ratings, completion status,
platforms, screenshots, and notes — all stored on your own machine, with no account or
network required.

## Features

- **Library** — separate *Owned* and *Interested* tabs, with release year, platforms, and genres.
- **Shelf** — a six-slot *Now Playing* shelf with drag-and-drop placement, reordering, and a removal flow.
- **Game details** — cover image, screenshot gallery with a featured shot, 1–10 rating,
  completed/dropped outcome, and free-form notes.
- **Platform ownership** — mark each platform for a game as owned or wanted.
- **Local storage** — SQLite database and imported images live in the Tauri app data
  directory. Nothing leaves your computer.

## Tech stack

| Layer    | Technology                          |
| -------- | ----------------------------------- |
| Desktop  | Tauri 2                             |
| Frontend | Vue 3, TypeScript, Pinia, Vite      |
| Backend  | Rust, rusqlite (bundled SQLite)     |

## Prerequisites

- [Node.js](https://nodejs.org/) 22 or newer
- [pnpm](https://pnpm.io/) 11 (Corepack can install it automatically)
- [Rust](https://www.rust-lang.org/tools/install) (stable, edition 2024)
- The [Tauri system dependencies](https://tauri.app/start/prerequisites/) for your OS

## Getting started

```bash
pnpm install
pnpm tauri dev
```

Build a production bundle with:

```bash
pnpm tauri build
```

## Scripts

| Command           | Description                                  |
| ----------------- | -------------------------------------------- |
| `pnpm dev`        | Run the Vite frontend only                   |
| `pnpm typecheck`  | Type-check the Vue/TypeScript sources        |
| `pnpm build`      | Type-check and build the frontend            |
| `pnpm test`       | Run the Rust test suite                      |
| `pnpm tauri dev`  | Run the desktop app in development           |
| `pnpm tauri build`| Build the desktop app bundle                 |

Rust formatting and lints:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

## Branching

- `main` — primary stable branch; only receives changes from `dev`.
- `dev` — integration branch; milestone branches merge here.
- `M<n>` — milestone work branches (currently `M6` for milestone #6).

New work lands on the current milestone branch, merges into `dev`, and `dev` is
merged into `main` once it is stable.

## Project structure

```
src/                  Vue frontend
  api/                Typed wrappers around Tauri commands
  components/         Grid, shelf, and dialog components
  composables/        Drag-and-drop, autosave, and hover helpers
  stores/             Pinia stores
  views/              Library and game detail views
src-tauri/            Rust backend
  migrations/         SQL migrations (append-only)
  src/commands/       Tauri command handlers
  src/db/             Database access layer
  dev/seed.sql        Optional sample data
```

## Sample data

The app creates its database on first launch. To load the sample library:

1. Launch the app once, then close it.
2. Run the seed script against the database in your app data directory, for example:

   ```bash
   sqlite3 "$HOME/.local/share/com.kiiro.backlogger/backlogger.db" < src-tauri/dev/seed.sql
   ```

   The directory is `%APPDATA%\com.kiiro.backlogger\` on Windows and
   `~/Library/Application Support/com.kiiro.backlogger/` on macOS.

## License

Backlogger is free software: you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

See [LICENSE](LICENSE) for the full text.
