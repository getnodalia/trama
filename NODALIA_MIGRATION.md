# Nodalia migration: Trama

Category: Creator suite app (with Lienzo and Pluma).
Status: named Trama, building from getnodalia, first NDL step, as of 2026-11-14. Evidence: the
pull request that adds this file.

## Current state

Pure-Rust interface design and prototyping app (egui), MIT OR Apache-2.0. Its code base began as
FigCraft (credited in `NOTICE`, `LICENSE-MIT` and About). Cargo git dependencies come from
getnodalia/foundation (craft-libs, egui and winit forks, craft-fonts) and getnodalia/nodalia-design
(`nodalia-theme`, `nodalia-update`). CI calls Foundation's reusable workflows pinned by commit, on
standard GitHub-hosted runners only; releases use Foundation's `release-tag` and
`release-publish` with Trama's own `release-linux.yml`, `release-macos.yml` (unsigned DMG) and
`release-windows.yml`. The weekly upstream sync renames FigCraft to Trama (`docs/upstream.md`).

## Desired state

No workflow, dependency or document depends on storytold or on an ArtCraft service; the interface
follows Foundation's [NDL standard](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md)
and the Creator suite's shared parts (`nodalia_theme::creator`, nodalia-design's
`docs/suites/creator.md`), as Lienzo and Pluma do.

## Done

- Names, app id `dev.danielmiguelt.Trama`, packages, installers and metadata say Trama; files,
  plugins and invite links made before the rename keep working.
- Original app icon (a woven swatch, Colloid style), the Linux hicolor set, `.ico` and the macOS
  renders; Trama's accent `#8B2FD6` and its DMG window background in nodalia-design.
- The theme: every chrome colour comes from NDL's light and dark palettes with Trama's accent.
  Canvas marks keep Trama's own colours (selection purple, components magenta so they never read
  as a selection).
- The title bar: NDL's round window controls on Linux and the BSDs (`TRAMA_TITLEBAR=desktop`
  keeps the desktop's frame), the system's on macOS, Windows' own caption buttons on Windows.
- The rail's main menu is NDL's hamburger.
- The in-app updater is `nodalia-update` against getnodalia/trama's releases.
- No default online-collaboration server: online sharing needs a server set in Preferences;
  sharing on the local network works as before.

## NDL still to do

In order. Each is its own pull request, with light and dark screenshots.

1. **Islands.** The panels (layers, properties, toolbar, tab strip) become rounded islands on the
   window colour with `nodalia_theme::component::{PANEL_RADIUS, WINDOW_GAP}`, the canvas rounded
   with `creator::round_island_corners`, as Pluma's `theme::bar_frame`.
2. **Top bar with the workspace selector** (`nodalia_theme::creator`): Design, Prototype and
   Inspect (Trama's dev mode) as workspaces, replacing the mode switch in the properties panel.
3. **Hamburger contents.** The rail's hamburger opens the full main menu today; NDL puts About,
   Preferences, Keyboard shortcuts and Quit first in it, in the header.
4. **Light and dark only in Preferences.** View ▸ Theme (`view.themeLight`, `view.themeDark`,
   `view.themeSystem`) leaves the menu; the commands stay for the CLI, MCP and the control channel,
   and Preferences keeps the choice.
5. **Home** on `nodalia_theme::creator::home` (rail, recents, new file), keeping Trama's online
   and local-network sections.
6. **Dialogs** (About, Preferences, Export, confirmations) with NDL's integrated controls
   (Preferences: close only) and the creator suite's buttons.
7. **Fonts.** The UI and the wordmark move to NDL's faces from craft-fonts.
8. **Global menu on Linux.** Export the menus over D-Bus to the shell's top-bar menu where it has
   one; the in-window menu bar shows only where it has none.
9. **Icons.** Move the shared UI icons (tools, align, boolean operations) to nodalia-design's set
   in the bold style.

## Technical debt

- The native file's signature (`"format": "figcraft"`), the plugin ABI (`figcraft`,
  `figcraft_run`), the old manifest name and the `figcraft://` scheme keep the old name for
  compatibility; `.github/rebrand.toml` keeps them.
- Internal Rust type names inherited from upstream (`FigApp`) are not product names and stay
  until upstream renames them, so the weekly sync merges cleanly.
- Upstream's `docs/brand/` and `docs/online-sharing-terms.md` were removed here; an upstream change
  to them shows up as a conflict in the sync pull request, resolved by keeping them removed.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, reusable workflows); getnodalia/nodalia-design
(theme, window controls, creator parts, updater, DMG background).
