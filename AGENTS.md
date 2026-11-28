# Trama: instructions for agents

Engineering rules for every Nodalia repository: [getnodalia/foundation](https://github.com/getnodalia/foundation)
([engineering](https://github.com/getnodalia/foundation/blob/main/standards/engineering.md),
[CI](https://github.com/getnodalia/foundation/blob/main/standards/ci.md),
[security](https://github.com/getnodalia/foundation/blob/main/standards/security.md),
[dependencies and licences](https://github.com/getnodalia/foundation/blob/main/standards/dependencies.md),
[release](https://github.com/getnodalia/foundation/blob/main/standards/release.md),
[NDL](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md)).

## What this is

Trama is Nodalia's interface design and prototyping app: a clean-room, open-source, pure-Rust
(egui) design tool with frames and auto layout, components and variants, styles and variables,
prototyping, inspection and live collaboration. It runs natively on macOS, Windows, Linux and
FreeBSD, and in the browser via WebAssembly. Licence: `MIT OR Apache-2.0`. It belongs to the
Nodalia **creator suite** with Lienzo and Pluma (`../lienzo`, `../pluma`): same conventions, same
shared parts from nodalia-design.

The product name is always **Trama** in user-facing text. Its code base began as an upstream
open-source project; those authors are credited in `NOTICE`, `LICENSE-MIT` and About, which must
stay as they are. Never describe Trama as a fork or a port in the README, the website, the UI or
release notes.

## Start every session here

1. Read [`ROADMAP.md`](ROADMAP.md) (stage, headline numbers) and [`docs/gaps.md`](docs/gaps.md) (the
   ranked work list). Unless the user gives you a task, pick work from the top of `docs/gaps.md`.
2. [`NODALIA_MIGRATION.md`](NODALIA_MIGRATION.md) lists what is still to move to Nodalia (NDL above
   all).
3. Orient → plan → implement + test → verify (screenshot) → record (ROADMAP, gaps) → commit.

`plan/` is an optional gitignored notebook. Do not ask for it, and do not stop when it is absent.

## Architecture

The crate map is in [`docs/architecture.md`](docs/architecture.md). Layering is enforced by
`cargo xtask layers`: geom/color (L0) → doc (L1) → layout/text/pathops/plugin/collab (L2) →
render/format (L3) → svg/pdf (L4) → engine (L5) → ui-egui/mcp (L6) → apps (`trama`, `trama-cli`,
`trama-server`, `trama-web`). Nothing below L6 depends on egui, eframe, rfd or winit. **The UI
crate is swappable.**

- **Everything is a command.** User-visible behaviour = a command in `crates/engine/src/cmd/*`
  (`spec!(id, label, params doc, undoable, run)`) + tests. The menus, shortcuts, Actions palette,
  CLI, control channel and MCP server all dispatch the same command ids.
- **Programmatic calls never open dialogs.** Commands only *request* UI (`Session::request_ui`); the
  control channel drops dialog requests.
- **The UI is thin**: panels read engine state and act through `app.run(id, params)` /
  `app.set_props(ids, props)`. Colours come from `theme::Tokens`, never hard-coded.
- Shared crates come from getnodalia/foundation (its `vendor/*` branches) pinned by commit; the
  Nodalia theme comes from nodalia-design. Never add a dependency on another organisation's
  repository.

## Build and test

`cargo xtask ci` is the gate CI runs (fmt, clippy `-D warnings`, tests, i18n report, layers,
assets, docs, wasm and the example plugins). Run it before every commit; while iterating, test the
crates you touched (`cargo test -p trama-engine`).

- `cargo run -p trama -- --control <port> [file.trama|file.svg|image]` opens the app with the JSON
  lines control channel (`docs/control-protocol.md`); pick a free port, sibling apps use the
  channel too. Then `{"id":1,"method":"ui.screenshot","params":{"path":"/tmp/shot.png"}}`.
- **For UI work, look at the result** (screenshot, read the PNG) in light and dark.
- Headless: `trama-cli` (`info`, `export`, `run`, `commands`, `mcp`; `docs/cli.md`, `docs/mcp.md`).
- Fonts are an optional build input: `NODALIA_FONTS_DIR` pointing at a checkout of the
  `nodalia/fonts` branch of getnodalia/foundation (`.cargo/config.toml` points it at
  `../nodalia-fonts`). Code and tests work without it.
- Parallel agents: separate `CARGO_TARGET_DIR`s, edit only the files you own, write manifests
  atomically, delete your target dir when done.

## Rules

### Never crash

- No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, `unimplemented!` in shipped code
  (workspace clippy lints deny them; tests may use them). Return `Result`/`Option`; use `.get()` for
  data-derived indexes; slice strings only at char boundaries; check floats are finite; cap sizes;
  bound recursion.
- `unsafe_code = "forbid"` workspace-wide. New workspace crates include `[lints] workspace = true`.
- `trama_engine::guard` turns a panicking command into `EngineError::Internal` and rolls the
  document back; the desktop app also guards each UI frame and writes recovery copies. They are
  safety nets, not substitutes for `Result`.
- Every crash fix comes with a regression test. The engine sweeps every command with junk
  parameters (`crates/engine/src/tests.rs`): extend fuzz and sweep tests when you add parsers or
  commands.

### Clean room

- Behaviour comes from public documentation and black-box observation of other design tools with
  synthetic documents only. Never read, disassemble or copy anything inside another product's
  application bundle, web bundle or network traffic.
- Never read or copy GPL/LGPL/AGPL code.

### Assets

- **No icon, image, illustration, cursor, logo, template, UI kit, font or screenshot from Figma or
  any other product**, and no lookalike redraws. Every asset is original (MIT OR Apache-2.0), OSI
  open source, public domain / CC0 or redistributable Creative Commons, and has a row in
  [`ATTRIBUTION.md`](ATTRIBUTION.md) in the same change (`cargo xtask assets` fails otherwise).
- UI icons are drawn in code (`crates/ui-egui/src/icons.rs`). The app icon is Nodalia's own
  (`assets/app-icon/`, `cargo xtask icon` renders the PNGs and `.ico` from `trama.svg`).
- Fonts live in getnodalia/foundation's `nodalia/fonts` branch, never in this repo.

### Compatibility

- Files saved before the rename keep working: the native file's signature stays
  `"format": "figcraft"`, `.figcraft` files open and save like `.trama` ones
  (`trama_format::OLDER_EXTENSIONS`), plugins keep their ABI (module `figcraft`, `figcraft_run`)
  and folders with the old manifest name still import.
- There is no upstream sync and no rename tooling: Trama is its own app, and the origin of its
  code base is credited in `NOTICE` and About.

### Working here

- A person answers for what their agent opens. `main` changes only through a pull request.
- Read the code before using it, keep fixes narrow, and add a test that fails without the fix.
- Report only what ran. Say what you could not test.
- Stop and ask only about licences outside the allowlist, publishing, releases, secrets, spending
  money, destructive git and anything else hard to undo.
- Keep secrets and personal details out of the repository, logs and requests.
- Dependencies and assets stay inside Foundation's
  [licence allowlist](https://github.com/getnodalia/foundation/blob/main/standards/dependencies.md).

## NDL

Trama's interface is egui and follows the Nodalia Design Language through `nodalia-theme` from
nodalia-design (creator suite: `docs/suites/creator.md` there). NDL's round window controls are
drawn on Linux only; macOS and Windows keep their native controls. What is still to move is in
`NODALIA_MIGRATION.md`.

## About window

- About is the shared Nodalia page from `nodalia_theme::about`: icon, name, version, links and a
  line that credits the upstream authors (see `NOTICE`). It has no Contributors or Models tabs,
  and there are no contributor or model lists compiled into the binary.

## CI workflows

- Workflows call getnodalia/foundation's reusable workflows pinned by commit; releases run on this
  repository's own `release-*.yml`.
- Pin every action to a full commit SHA, give each job the least permissions it needs, and use only
  standard GitHub-hosted runners (zero cost).
- Keep CI changes in their own pull request unless a change cannot work without them.

## Release

Merging a version bump with its notes (`docs/release-notes/vX.Y.Z.md`, format in that folder's
README) releases it: `release-auto.yml` tags the merge through Foundation's `release-tag`, builds
the packages with `release-linux.yml`, `release-macos.yml` (unsigned DMG on a standard macOS runner)
and `release-windows.yml`, and publishes the release. `main` carries a `-dev` version between
releases. Nobody pushes a tag by hand.
