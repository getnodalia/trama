# Releasing Trama

Releases are automatic. A pull request that sets `[workspace.package] version` in the root
`Cargo.toml` to a release version (no `-dev`) and adds its notes in
`docs/release-notes/vX.Y.Z.md` (format in [that folder's README](release-notes/README.md); the
**Release notes are written** check runs `scripts/check-release-notes.sh`) is all a release
takes. Once it merges, `release-auto.yml` tags the merge through Foundation's `release-tag`
workflow, builds the packages with this repository's `release-linux.yml`, `release-macos.yml` and
`release-windows.yml`, and Foundation's `release-publish` publishes the release with those notes.
Nobody pushes a tag by hand. Afterwards `main` goes back to the next `-dev` version.

User-facing names say **Trama**. Files, binaries and ids stay lowercase
(`trama-<version>-<platform>-<arch>.<ext>`, `dev.danielmiguelt.Trama`).

## Trama's names

| Thing | Value |
|---|---|
| Display name | Trama |
| Packages / binaries | `trama` (app), `trama-cli` (headless CLI), `trama-web` (browser) |
| Bundle / app id | `dev.danielmiguelt.Trama` |
| Native document | `.trama`, MIME `application/x-trama`, UTI `dev.danielmiguelt.Trama.document` |
| Also opens | `.svg`, `.png`, `.jpg`/`.jpeg` ("Open with", never the default app) |
| WiX UpgradeCode | `33A433A8-CFE1-4969-ACAD-171AAB11DE3E` (fixed forever; never reuse it in another app) |
| Linux categories | `Graphics;2DGraphics;VectorGraphics;X-Design;` (`Design` isn't a registered freedesktop category, so `desktop-file-validate` rejects it) |

## Cutting a release

1. **Bump the version** in a pull request:

   ```sh
   cargo xtask version              # prints the current version, e.g. 0.1.0-dev
   cargo xtask version set 0.1.0    # or 0.1.0-rc.1; updates Cargo.toml and Cargo.lock
   ```

2. **Write the notes** in `docs/release-notes/v0.1.0.md`, from the merged pull requests.
3. **Merge** once the required checks pass. The release builds and publishes itself; a version
   with a pre-release suffix (`-rc.1`) is marked pre-release.
4. **Set `main` back** to the next `-dev` version (`cargo xtask version set 0.1.1-dev`).

A failed build leaves the tag on a draft; re-running the failed job finishes it.

## The default server

Share ▸ Online uses the collaboration server set in Preferences. Trama ships **without** a
default server (`trama_engine::live::DEFAULT_SERVER` is empty), so online sharing stays off until
the user enters one; sharing on the local network works either way. A distribution that runs its
own server can build it in with `TRAMA_DEFAULT_SERVER`:

```sh
TRAMA_DEFAULT_SERVER=wss://design.example.org/ cargo build --release -p trama
```

How to run a server is in [`server.md`](server.md). The release workflows leave it unset.

## The default font

New text starts in Inter (`trama_doc::DEFAULT_FONT`). A distribution whose users mostly write a
script Inter lacks (Thai, say) can name another family with `TRAMA_DEFAULT_FONT` when building;
unset or empty means Inter. Pick a family the build embeds from nodalia-fonts (the web build has no
other fonts) or one its users have installed:

```sh
TRAMA_DEFAULT_FONT="Noto Sans Thai" cargo build --release -p trama
```

Files record each text's family, so the variable only changes what new text starts in: a file
saved by one build keeps its fonts in a build with another default. Like any other family, it
renders only where it is available (embedded or installed); elsewhere the text falls back to the
faces that are. The release workflow leaves it unset.

## What gets built

Every job runs on a standard GitHub-hosted runner, at no cost.

| Platform | Artifacts | Built on |
|---|---|---|
| Linux x86_64 | `trama-<v>-linux-x86_64.{AppImage,AppImage.zsync,deb,rpm,tar.gz}` | `ubuntu-22.04` |
| macOS 11+ (universal) | `trama-<v>-macos-universal.dmg`, unsigned | `macos-15` |
| Windows 10+ x64 | `trama-<v>-windows-x64.msi`, `trama-<v>-windows-x64-portable.zip`, unsigned | `windows-2025` |

Each workflow checks what it built before attaching it: the AppImage must start on bare Ubuntu,
Debian, Fedora and Arch images, and the portable Windows build must report its version.
`trama-cli` ships inside every desktop package. `trama --version` prints the version and, in CI
builds, the commit (`TRAMA_BUILD_SHA`, read by `apps/trama/build.rs`).

The collaboration server (`apps/trama-server`, [`server.md`](server.md)) and the web build
(`apps/trama-web`) are not release artifacts yet; build them from source as described below.

**Fonts.** Each release job checks out
[nodalia-fonts](https://github.com/getnodalia/foundation/tree/nodalia/fonts) at a pinned
commit and builds with `NODALIA_FONTS_DIR` and `NODALIA_FONTS_REQUIRED=1`, so releases embed its
fonts and fail rather than ship without them. Packages carry each embedded font's licence as
`OFL-<family>.txt` (`copy_font_licences` in `packaging/env.sh`; `Contents/Resources/Licenses` in
the macOS app). Source Sans 3 and Source Serif 4 are filtered out on every target under Trama's
asset policy.

### macOS

`packaging/macos/package.sh [--arch universal|aarch64|x86_64]` builds both Apple targets with
`MACOSX_DEPLOYMENT_TARGET=11.0`, joins them with `lipo` and assembles `Trama.app`
(`Contents/MacOS/Trama`, `Contents/Resources/Trama.icns`, `Info.plist` from `Info.plist.in`).
The `.icns` is generated at packaging time with `iconutil` from the committed
`assets/app-icon/trama-<size>.png` (made by `cargo xtask icon`). The app gets an ad-hoc
signature and is neither signed with a Developer ID nor notarized: on first launch, macOS asks
the user to open it from the context menu (or System Settings ▸ Privacy & Security).

The DMG opens on Trama's own install window: the background and the window layout
(`packaging/macos/dmg/`, made by its `generate.py` from Nodalia's shared DMG template) place the
app and the `Applications` link.

```sh
packaging/macos/package.sh --arch aarch64     # quick, host-only; omit --arch for universal
hdiutil attach -nobrowse -readonly dist/release/trama-*-macos-aarch64.dmg   # inspect, then detach
```

### Windows

`packaging/windows/package.ps1 -Arch x64|x86|arm64` builds with `+crt-static` (no VC++
redistributable), checks each exe's PE machine and subsystem (app GUI, CLI console), builds the
MSI with WiX v5 (`trama.wxs`: per-machine, Start Menu shortcut, owner of `.trama`, "Open with"
for SVG/PNG/JPEG, App Paths, setup wizard with art drawn from the app icon), validates ICE50 and
zips a portable build. The files are unsigned, so SmartScreen may warn on first run.
`apps/trama/build.rs` embeds the icon and version info with `winresource`.

Locally on Windows: `dotnet tool install -g wix --version 5.0.2`,
`wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2`, then
`pwsh packaging/windows/package.ps1 -Arch x64`.

### Linux

`packaging/linux/package.sh [--formats "appimage deb rpm tar"]` stages one FHS tree (binaries,
`dev.danielmiguelt.Trama.desktop`, hicolor icons 16–512 px, AppStream metainfo, a
shared-mime-info file for `.trama`) and builds every format from it: `.tar.gz`; `.deb` and
`.rpm` with [nfpm](https://nfpm.goreleaser.com/); and an `.AppImage` with update information and
a `.zsync`, whose `AppRun` registers the desktop entry and icons on launch so Wayland docks show
the right icon (`TRAMA_NO_DESKTOP_INTEGRATION=1` opts out; covered by `apprun-test.sh`) and runs
the bundled `trama-cli` for `Trama.AppImage cli <args>`. Binaries are built on Ubuntu 22.04 and
need glibc ≥ 2.35. `packaging/linux/flatpak/` keeps the manifests for a future Flathub
submission; they are not built in CI.

### FreeBSD

`packaging/freebsd/package.sh` writes a `/usr/local`-style tarball, and
`packaging/freebsd/package.sh --dry-run` checks the layout on any OS with stub binaries
(packaging-lint runs it). `.github/workflows/freebsd.yml` builds and tests on FreeBSD.

### Web

`packaging/web/package.sh` runs `trunk build --release` in `apps/trama-web` (`Trunk.toml`:
`public_url = "./"`, output `dist/web`), fails if a `.wasm` exceeds 24 MiB, and zips the site
with sample `_headers`/`.htaccess` and the hosting guide. The app uses WebGPU with a WebGL2
fallback; `?webgl` forces WebGL2. 0.1.0: wasm 17.7 MiB raw, 6.7 MiB gzip, 4.4 MiB Brotli.
`apps/trama-web/.cargo/config.toml` builds it with wasm SIMD (`+simd128`), which keeps the
glyph rasterizer's scalar fallback out of the module.

The wasm is built with the size-optimised `web-release` Cargo profile (`opt-level = "s"`, fat
LTO, one codegen unit; `data-cargo-profile` in `apps/trama-web/index.html`) and then
`wasm-opt -Oz`, which needs the wasm features rustc emits by default passed explicitly
(`data-wasm-opt-params`). Sizes: 0.2.0 21.1 MiB and 0.3.0 25.8 MiB (failed the gate; the plugin
interpreter and collaboration code landed) with the desktop `release` profile; 0.3.1 17.2 MiB
with `web-release`. The desktop builds keep `release`.

## Checks

- `.github/workflows/ci.yml` (pull requests and `main`): Foundation's Rust gate (fmt, clippy
  `-D warnings`, tests, layering, asset attribution, wasm32 and the example plugins), the
  licence allowlist (`cargo deny check licenses` against `deny.toml`), git dependencies only
  from getnodalia, and no upstream names (`python3 scripts/rebrand.py check`).
- `.github/workflows/packaging-lint.yml` (changes to `packaging/` or workflows): shellcheck, the
  AppRun test, a PowerShell parse, the FreeBSD dry run, xmllint, `desktop-file-validate` and
  `appstreamcli validate`.
- `.github/workflows/release-notes.yml`: a release version needs its notes.
