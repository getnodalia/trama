# Trama app icon

**Figure:** Joseph Marie Jacquard (1752–1834). His loom read punched cards to weave a picture
thread by thread, row by row: a design laid on a grid, as an interface is (*trama* is the weft).

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, the bust running off the bottom edge,
in the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** Trama purple, `#9A45E2` at the top to `#7A26C9` at the bottom; 512 × 512 viewBox,
rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is a wood engraving of Jacquard signed Bocourt and Tourfaut, from
"Les bienfaiteurs de l'Humanité" (Paris, Librairie Paul Ducrocq, 1878), on
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Joseph_Marie_Jacquard-portrait.jpg)
(public domain). The figure was traced out of its hatched background, converted to black and white and placed over the app colour with nodalia-design's
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
# (the 1280 px render of the Commons file; --mask traces the figure, since the model keeps the
# engraving's hatched background)
python3 tools/app-portrait/portrait.py "Joseph_Marie_Jacquard-portrait.jpg" trama.png \
  --box 210,170,1090,1050 --colors '#9A45E2' '#7A26C9' --gamma 1.25 \
  --mask 180,1300,230,1000,260,860,330,780,420,700,452,630,450,560,442,480,468,400,512,325,568,262,640,238,708,240,770,272,822,330,858,410,872,495,864,560,846,612,880,660,960,730,1030,820,1080,940,1110,1100,1130,1300
# 2. the SVGs, here:
python3 scripts/icon-svg.py trama.png assets/app-icon
# 3. every render (needs resvg: cargo install resvg):
cargo xtask icon
```

The SVGs embed a raster portrait instead of traced paths, so a regenerated set can differ slightly
from the committed files in resampling.
