# Trama app icon

**Figure:** Joseph Marie Jacquard (1752–1834). His loom read punched cards to weave a picture
thread by thread, row by row: a design laid on a grid, as an interface is (*trama* is the weft).

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders,
in the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** Trama purple, `#9A45E2` at the top to `#7A26C9` at the bottom; 512 × 512 viewBox,
rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "A la mémoire de J.M. Jacquard" (1839), the woven-silk portrait
made on a Jacquard loom by Michel Marie Carquillat after a painting by Claude Bonnefond, from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:A_la_m%C3%A9moire_de_J.M._Jacquard.jpg)
(public domain). The bust was cropped from the seated figure, separated from its background,
converted to black and white and placed over the app colour with nodalia-design's
`tools/app-portrait`. Licence: see [LICENSE.txt](LICENSE.txt).

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
# 1. the artwork, in nodalia-design (needs rembg):
python3 tools/app-portrait/portrait.py "A_la_mémoire_de_J.M._Jacquard.jpg" trama.png \
  --box 430,322,810,702 --colors '#9A45E2' '#7A26C9' --gamma 1.35
# 2. the SVGs, here:
python3 scripts/icon-svg.py trama.png assets/app-icon
# 3. every render (needs resvg: cargo install resvg):
cargo xtask icon
```

The SVGs embed a raster portrait instead of traced paths, so a regenerated set can differ slightly
from the committed files in resampling.
