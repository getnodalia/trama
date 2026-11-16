# Attribution

Every asset bundled in this repository (images, icons, fonts, data files) is listed here with its
author, source and licence. `cargo xtask assets` fails if a file under `assets/` or
`docs/images/` is missing from this list.

Trama's user-interface icons are drawn in code (`crates/ui-egui/src/icons.rs`): original work,
MIT OR Apache-2.0. No third-party icon sets are used.

## Assets in this repository

| Path | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/` (all files: `trama.svg`, `trama-linux.svg`, `trama-*.png`, `trama.ico`, `hicolor/`) | Nodalia; source portrait: wood engraving signed Bocourt and Tourfaut, 1878 | Portrait: ["Joseph Marie Jacquard-portrait.jpg"](https://commons.wikimedia.org/wiki/File:Joseph_Marie_Jacquard-portrait.jpg), from "Les bienfaiteurs de l'Humanité" (1878), Wikimedia Commons; cut out, black and white, on Trama's purple; SVGs by `scripts/icon-svg.py`, derived files by `cargo xtask icon` | MIT OR Apache-2.0 (derived image), [`assets/app-icon/LICENSE.txt`](assets/app-icon/LICENSE.txt); source portrait: public domain | Trama app icon (portrait of Joseph Marie Jacquard) |
| `packaging/macos/dmg/` (`background.svg`, `background.tiff`, `DS_Store`) | Nodalia (drawn by `tools/dmg-background/draw_background.py` in [nodalia-design](https://github.com/getnodalia/nodalia-design); text outlined from Sora, OFL-1.1) | Original work; `background.tiff` and `DS_Store` written by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 (Sora outlines: OFL-1.1) | The macOS DMG window |
| `docs/images/` (`art-*.png`) | FigCraft contributors | PNG exports by Trama's renderer of the synthetic demo documents in `examples/` | MIT OR Apache-2.0 | No third-party artwork |
| `docs/images/` (`ui-*.png`) | Nodalia | Screenshots of Trama in Nodalia Design (light and dark), showing the synthetic demo documents in `examples/`, taken through the control channel | MIT OR Apache-2.0 | Original; no third-party artwork |
| `examples/*.trama` | FigCraft contributors | Synthetic demo documents built through Trama's command API; invented copy, shapes and vectors drawn from scratch, no images or embedded fonts | MIT OR Apache-2.0 | Text uses Inter (OFL, from craft-fonts or the system); no third-party artwork |

## Fonts (build input, not in this repository)

Release builds embed fonts from the optional craft-fonts build input
([getnodalia/foundation (vendor/craft-fonts/main)](https://github.com/getnodalia/foundation/tree/vendor/craft-fonts/main), `CRAFT_FONTS_DIR`). Its own
`ATTRIBUTION.md` and licence files are authoritative; the packages ship each font's licence.

| Family | Licence | Upstream |
|---|---|---|
| Inter | SIL Open Font License 1.1 | https://github.com/rsms/inter |
| Archivo | SIL Open Font License 1.1 | [Google Fonts / Archivo](https://github.com/google/fonts/tree/6c70c829f09ea345d3590406693220ea35c6553f/ofl/archivo) |
| Roboto, Montserrat, Lato, Open Sans, Poppins, Nunito Sans, Merriweather, Playfair Display, Carlito, Caladea | SIL Open Font License 1.1 | Google Fonts / upstream repositories (see craft-fonts) |
| Liberation Sans, Serif, Mono | SIL Open Font License 1.1 | https://github.com/liberationfonts |
| BIZ UDPGothic, BIZ UDMincho, Shippori Mincho, Noto Sans Arabic, Noto Sans Georgian | SIL Open Font License 1.1 | Google Fonts (see craft-fonts) |

The central catalog also contains Adobe-authored Source Sans 3 and Source Serif 4.
Trama filters both families from every build in accordance with its asset policy.

egui's default fonts (compiled into the `egui`/`epaint` crates) keep their own licences (OFL / Apache-2.0 / Ubuntu Font Licence); see those crates.

## egui dependency font icons

| Asset | Author | Source | Licence | Use |
|---|---|---|---|---|
| `epaint_default_fonts::EGUI_ICONS`, `epaint_default_fonts::EMOJI_ICON` (dependency font inputs) | John Slegers | [egui 6d518fc7](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts), [emoji-icon-font](https://github.com/jslegers/emoji-icon-font) | MIT (Copyright (c) 2014 John Slegers; full upstream text in `NOTICE`) | egui-icons.ttf is the five-glyph subset, included by default_fonts independently of monochrome_emoji_fonts; emoji-icon-font.ttf is the full monochrome fallback. Dependency assets unmodified here; no font file copied into this repository. |

## Arabic interface catalog

| Path | Contributor | Source | Licence | Review |
|---|---|---|---|---|
| `crates/ui-egui/locales/ar.toml` | @hussain-hz with OpenAI Codex (AI-assisted) | Original Arabic wording from Trama's English catalog and engine command labels; no proprietary localisation consulted | MIT OR Apache-2.0 | Draft; native-speaker review and maintainer approval pending |
