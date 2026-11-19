# Trama app icon

**Figure:** Charles Babbage (1791–1871). His Analytical Engine took its programs from the punched cards of
the Jacquard loom, which wove a picture thread by thread, row by row: a design laid on a grid, as an
interface is (*trama* is the weft).

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, the bust running off the bottom edge,
in the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** Trama purple, `#9A45E2` at the top to `#7A26C9` at the bottom; 512 × 512 viewBox,
rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is the photograph "Charles Babbage - 1860.jpg" (author unknown) on
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Charles_Babbage_-_1860.jpg)
(public domain). The figure was cut out with a portrait matte, converted to black and white and placed over the app colour (nodalia-design's
`tools/app-portrait`). Licence: see [LICENSE.txt](LICENSE.txt).

## Files

| File | What |
|---|---|
| `trama.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `trama-linux.svg` | the Colloid grid used by the Linux icon theme and nodalix-icon-theme: a 224 tile at 16 on a 256 canvas, corner radius 51.5, with a soft shadow below |
| `trama-<size>.png` (16–1024) | renders on the macOS grid; `packaging/macos/package.sh` builds the `.icns` from them |
| `trama.ico` | Windows icon, 16–256 px, embedded in the `.exe` by `apps/trama/build.rs` |
| `hicolor/<size>/apps/dev.danielmiguelt.Trama.png` (16–512), `hicolor/scalable/…svg` | Linux icon theme (from `trama-linux.svg`); the 256 px one is also the runtime window icon off macOS |

## Regenerate

```sh
# 1. the artwork, in nodalia-design (the 4478 px photograph from Commons; see candidates/README.md
#    there for the cut-out settings):
# 2. the SVGs, here:
python3 scripts/icon-svg.py trama.png assets/app-icon
# 3. every render (needs resvg: cargo install resvg):
cargo xtask icon
```

The SVGs embed a raster portrait instead of traced paths, so a regenerated set can differ slightly
from the committed files in resampling.
