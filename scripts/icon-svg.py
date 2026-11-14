#!/usr/bin/env python3
"""Writes Trama's icon SVGs into the given folder (assets/app-icon): the 512 full-bleed master
`trama.svg` and the 256 Colloid-grid Linux icon `trama-linux.svg`. Then run `cargo xtask icon`."""
import sys
from pathlib import Path

out = Path(sys.argv[1])
TOP, BOTTOM = "#9A45E2", "#7A26C9"
WEFT, WARP = "#FFC24D", "#FFFFFF"

# The glyph, on a 224 tile whose corner is at (0, 0): three amber weft threads across, three white
# warp threads down, woven: at each crossing one goes over, alternating like plain-weave cloth.
def glyph(grad):
    w, g = 24, 6                    # thread width, the gap the over thread leaves on the under one
    pos = [58, 112, 166]            # thread centres
    lo, hi = 20, 204                # thread ends
    out = ['  <g>']
    for y in pos:
        out.append(f'    <rect x="{lo}" y="{y-w/2}" width="{hi-lo}" height="{w}" rx="{w/2}" fill="{WEFT}"/>')
    for x in pos:
        out.append(f'    <rect x="{x-w/2}" y="{lo}" width="{w}" height="{hi-lo}" rx="{w/2}" fill="{WARP}"/>')
    for i, y in enumerate(pos):
        for j, x in enumerate(pos):
            if (i + j) % 2 == 0:
                # The weft goes over here: the warp shows a gap on each side of it.
                out.append(f'    <rect x="{x-w/2-0.5}" y="{y-w/2-g}" width="{w+1}" height="{w+2*g}" fill="url(#{grad})"/>')
                out.append(f'    <rect x="{x-w/2-g-1}" y="{y-w/2}" width="{w+2*g+2}" height="{w}" fill="{WEFT}"/>')
            else:
                # The warp goes over: the weft shows the gap.
                out.append(f'    <rect x="{x-w/2-g}" y="{y-w/2-0.5}" width="{w+2*g}" height="{w+1}" fill="url(#{grad})"/>')
                out.append(f'    <rect x="{x-w/2}" y="{y-w/2-g-1}" width="{w}" height="{w+2*g+2}" fill="{WARP}"/>')
    out.append('  </g>')
    return "\n".join(out)

COMMENT = '''<!-- Trama app icon: a swatch of plain-weave cloth, the weft ("trama") in amber across and the
     warp in white down, each crossing alternating over and under: threads laid on a grid, as an
     interface is. Original work for Nodalia,
     MIT OR Apache-2.0 (assets/app-icon/README.md). Plain vector shapes, no embedded images. -->'''

def edges(r):
    # Colloid's lit top edge and shaded bottom edge, on a 224 tile with corner radius r.
    s = 224
    return (f'  <path d="M0 {s-r-4}v4c0 {r:.1f} {r*0+0:.0f} 0 0 0" fill="none"/>' if False else
            f'  <path d="M0 {s-r}v-3.5a{r} {r} 0 0 0 {r} {r}h{s-2*r}a{r} {r} 0 0 0 {r}-{r}v3.5a{r} {r} 0 0 1-{r} {r}h-{s-2*r}a{r} {r} 0 0 1-{r}-{r}z" fill="#000" opacity=".15"/>\n'
            f'  <path d="M0 {r}v3.5a{r} {r} 0 0 1 {r}-{r}h{s-2*r}a{r} {r} 0 0 1 {r} {r}v-3.5a{r} {r} 0 0 0-{r}-{r}h-{s-2*r}a{r} {r} 0 0 0-{r} {r}z" fill="#fff" opacity=".35"/>')

def tile_defs(id_, y0, y1):
    return f'''    <linearGradient id="{id_}" x1="0" y1="{y0}" x2="0" y2="{y1}" gradientUnits="userSpaceOnUse">
      <stop offset="0" stop-color="{TOP}"/>
      <stop offset="1" stop-color="{BOTTOM}"/>
    </linearGradient>'''

# Master: 512 full-bleed, the 224 design scaled by 512/224, corner radius 112 (= 49 on 224).
k = 512 / 224
r_master = 112 / k
master = f'''<?xml version="1.0" encoding="UTF-8"?>
{COMMENT}
<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512">
  <defs>
{tile_defs("tile", 0, 224)}
  </defs>
  <g transform="scale({k:.6f})">
  <rect width="224" height="224" rx="{r_master:.4f}" fill="url(#tile)"/>
{edges(round(r_master, 4))}
{glyph("tile")}
  </g>
</svg>
'''

# Linux: Colloid's grid, a 224 tile at 16 on a 256 canvas, radius 51.5, a soft shadow 5 below.
shadow = "\n".join(
    f'  <rect x="{16 - d:.1f}" y="{16 + 5 - d * 0.4:.1f}" width="{224 + 2 * d:.1f}" height="{224 + 2 * d:.1f}" rx="{51.5 + d:.1f}" fill="#000" opacity="{o}"/>'
    for d, o in [(3.0, ".03"), (2.0, ".04"), (1.0, ".05"), (0.0, ".06")]
)
linux = f'''<?xml version="1.0" encoding="UTF-8"?>
{COMMENT}
<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">
  <defs>
{tile_defs("tile", 0, 224)}
  </defs>
{shadow}
  <g transform="translate(16 16)">
  <rect width="224" height="224" rx="51.5" fill="url(#tile)"/>
{edges(51.5)}
{glyph("tile")}
  </g>
</svg>
'''
(out / "trama.svg").write_text(master)
(out / "trama-linux.svg").write_text(linux)
