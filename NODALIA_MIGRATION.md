# Nodalia migration: Trama

Category: Creator suite app (with Lienzo and Pluma).
Status: named Trama, building from getnodalia, NDL interface, as of 2026-11-15. Evidence: the
pull request that adds this file.

## Current state

Pure-Rust interface design and prototyping app (egui), MIT OR Apache-2.0. Its code base began as
FigCraft (credited in `NOTICE`, `LICENSE-MIT` and About). Cargo git dependencies come from
getnodalia/foundation (shared libraries, egui and winit, fonts) and getnodalia/nodalia-design
(`nodalia-theme`, `nodalia-update`). CI calls Foundation's reusable workflows pinned by commit, on
standard GitHub-hosted runners only; releases use Foundation's `release-tag` and
`release-publish` with Trama's own `release-linux.yml`, `release-macos.yml` (unsigned DMG) and
`release-windows.yml`. There is no upstream sync.

## Desired state

No workflow, dependency or document depends on a repository or service outside Nodalia; the interface
follows Foundation's [NDL standard](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md)
and the Creator suite's shared parts (`nodalia_theme::creator`, nodalia-design's
`docs/suites/creator.md`), as Lienzo and Pluma do.

## Done

- Names, app id `dev.danielmiguelt.Trama`, packages, installers and metadata say Trama; files,
  plugins and invite links made before the rename keep working.
- App icon: Charles Babbage's portrait on Trama's purple (Nodalia icon family), the Linux hicolor set, `.ico` and the macOS
  renders; Trama's accent `#8B2FD6` and its DMG window background in nodalia-design.
- The theme: every chrome colour comes from NDL's light and dark palettes with Trama's accent.
  Canvas marks keep Trama's own colours (selection purple, components magenta so they never read
  as a selection).
- The title bar: NDL's round window controls on Linux and the BSDs (`TRAMA_TITLEBAR=desktop`
  keeps the desktop's frame), the system's on macOS, Windows' own caption buttons on Windows.
- The in-app updater is `nodalia-update` against getnodalia/trama's releases.
- No default online-collaboration server: online sharing needs a server set in Preferences;
  sharing on the local network works as before.

## NDL: done

- **Islands.** The rail and the layers panel, the properties panel and the canvas are rounded
  islands (`creator::layout::ISLAND_RADIUS`) on the window's colour, 8 pt apart; a panel's edge is
  grabbed in the gap between islands. With the interface hidden or minimized, and in a phone-sized
  window, the canvas has the whole window.
- **Top bar** at the creator suite's height (`creator::layout::APP_BAR`): NDL's hamburger, Home,
  the file tabs and, while a design file shows, the creator workspace selector (Design,
  Prototype, Inspect) left of the window controls.
- **Hamburger.** The main menu moved from the rail to the top bar; it holds the menus (when the
  desktop has no global menu) and ends with About, Preferences, Keyboard shortcuts and Quit.
- **Light and dark only in Preferences.** The bar's theme toggle and View ▸ Theme are gone; the
  commands stay for the CLI, MCP and the control channel.
- **Home** is one island on the window's colour, with NDL's control radius on its sidebar rows.
- **About and Preferences** carry NDL's integrated close control and no system title bar; every
  floating window has the island radius.
- **Icons.** Toolbar, panel and menu icons draw NDL's bold glyphs; Trama's own drawings stay for
  what NDL has no glyph for (components, auto layout, corners, gradients, live and window marks).
- **Global menu on Linux.** The menus are exported over D-Bus (`com.canonical.dbusmenu`,
  registered with `com.canonical.AppMenu.Registrar`) to the shell's top-bar menu where it has one;
  the hamburger then shows only the app's own entries.

## NDL still to do

1. **Fonts.** The UI and the wordmark move to NDL's faces from nodalia-fonts.
2. **Home on `creator::home`.** Home keeps Trama's own layout (online and local-network sections)
   on its island; moving it onto the shared creator Home needs those sections there first.
3. **The other dialogs** (Export, confirmations) on the creator frame with its buttons.
