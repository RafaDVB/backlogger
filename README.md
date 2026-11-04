# Backlogger

A local-first desktop app for tracking your video game backlog, wishlist and on-going playthroughs. Inspired by Ross Scott's
Big Game List in Accursed farms and a video on the "rule of three", where you pick three games, three movies and three books
for the entire month. This app is focused on the gaming side.

## Features

- **Library** — separate tabs for games you owned and ones you're interested in, with release year, platforms, and genres.
- **Shelf** — a six-slot virtual shelf with drag-and-drop placement, reordering, and a removal flow.
- **Game details** — cover image, screenshot gallery with a featured shot, customizable rating,
  completed/dropped status, and free-form notes.
- **Platform ownership** — mark each platform for a game as owned or wanted.
- **Local storage** — SQLite database and imported images live in your computer. Specifically in %appdata% or ~/.local/share

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
| `pnpm test`       | Run the Rust and vitest test suites          |
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
   `~/.local/share/com.kiiro.backlogger/` on Linux.

## License

Copyright (C) 2026 RafaDVB.

Backlogger is free software: you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

Backlogger is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See [LICENSE](LICENSE) for the full text.

Third-party components and their license texts are listed in
[THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md). Regenerate that file with
`pnpm licenses:generate` after changing dependencies.

## Credits

- [Pixel Operator](https://www.dafont.com/pixel-operator.font) font by Jayvee
  Enaguas (HarvettFox96), released under CC0 1.0.
