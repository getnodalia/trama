# Trama app icon

A **swatch of plain-weave cloth** on a purple tile: three amber weft threads across (the *trama*,
the weft, that gives the app its name) and three white warp threads down, each crossing
alternating over and under. Threads laid on a grid, as an interface is.

## Palette

| Colour | Hex | Used for |
|---|---|---|
| Trama purple, top | `#9A45E2` | the tile's gradient |
| Trama purple, bottom | `#7A26C9` | the tile's gradient |
| Amber | `#FFC24D` | the weft |
| White | `#FFFFFF` | the warp |

The tile follows Colloid: a soft vertical gradient, a lit top edge and a shaded bottom edge. No
letters, no raster images: every shape is a plain SVG rectangle or path.

## Geometry

- `trama.svg`: the master, a 512-unit tile (`viewBox="0 0 512 512"`) edge to edge, corner radius
  112. The macOS renders pad it onto Apple's 824/1024 icon grid; Windows uses it as it is.
- `trama-linux.svg`: the Colloid grid used by the Linux icon theme and nodalix-icon-theme, a 224
  tile at 16 on a 256 canvas, corner radius 51.5, with a soft shadow below.

Both come from `scripts/icon-svg.py`, which draws the weave on the 224 grid.

## Files

- `trama-<size>.png` (16–1024): renders on the macOS grid; `packaging/macos/package.sh` builds
  the `.icns` from them.
- `trama.ico`: Windows icon, 16–256 px, embedded in the `.exe` by `apps/trama/build.rs`.
- `hicolor/<size>/apps/dev.danielmiguelt.Trama.png` (16–512) and `hicolor/scalable/...svg`:
  Linux icon theme (from `trama-linux.svg`); the 256 px one is also the runtime window icon off
  macOS.

## Regenerate

`python3 scripts/icon-svg.py assets/app-icon`, then `cargo xtask icon` (needs `resvg`:
`cargo install resvg`).

## Provenance

Original work made for Trama by Nodalia. Licence: see `LICENSE.txt`.
