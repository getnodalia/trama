#!/usr/bin/env python3
"""Writes Trama's icon SVGs from its portrait artwork: the 512 full-bleed master `trama.svg` and
the 256 Colloid-grid Linux icon `trama-linux.svg`. Then run `cargo xtask icon`.

    python3 scripts/icon-svg.py PORTRAIT.png assets/app-icon

PORTRAIT is the 1024 px full-bleed artwork (the portrait already cut out and laid on Trama's
purple), made by nodalia-design's tools/app-portrait (see assets/app-icon/README.md). The master
SVG embeds it, so it is the canonical artwork afterwards. Needs Pillow.
"""
import base64
import io
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

QUALITY = 90


def jpeg(image, size):
    buffer = io.BytesIO()
    image.convert('RGB').resize((size, size), Image.LANCZOS).save(
        buffer, 'JPEG', quality=QUALITY, optimize=True, progressive=True)
    return base64.b64encode(buffer.getvalue()).decode()


def master(art, comment):
    """The 512 full-bleed tile: the portrait clipped to a rounded square, corner radius 112."""
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512">\n'
        f'  {comment}\n'
        '  <defs><clipPath id="tile"><rect width="512" height="512" rx="112"/></clipPath></defs>\n'
        f'  <image width="512" height="512" clip-path="url(#tile)" href="data:image/jpeg;base64,{jpeg(art, 1024)}"/>\n'
        '</svg>\n')


def shadow():
    # Colloid's soft shadow 5 px below the tile, drawn as an image: not every icon renderer
    # applies SVG blur filters.
    scale, canvas, margin, tile = 2, 256, 16, 224
    mask = Image.new('L', (canvas * scale, canvas * scale), 0)
    top = (margin + 5) * scale
    ImageDraw.Draw(mask).rounded_rectangle(
        (margin * scale, top, (margin + tile) * scale, top + tile * scale),
        radius=51.5 * scale, fill=round(255 * 0.144))
    mask = mask.filter(ImageFilter.GaussianBlur(3.5 * scale))
    layer = Image.new('LA', mask.size, 0)
    layer.putalpha(mask)
    buffer = io.BytesIO()
    layer.save(buffer, 'PNG', optimize=True)
    return base64.b64encode(buffer.getvalue()).decode()


def linux(art, comment):
    """Colloid's application grid: a 224 tile at 16 on a 256 canvas, radius 51.5, soft shadow."""
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">\n'
        f'  {comment}\n'
        '  <defs><clipPath id="tile"><rect x="16" y="16" width="224" height="224" rx="51.5"/></clipPath></defs>\n'
        f'  <image width="256" height="256" href="data:image/png;base64,{shadow()}"/>\n'
        f'  <image x="16" y="16" width="224" height="224" clip-path="url(#tile)" href="data:image/jpeg;base64,{jpeg(art, 448)}"/>\n'
        '</svg>\n')


COMMENT = ('<!-- Trama app icon: Joseph Marie Jacquard, from the woven-silk portrait "A la mémoire de '
           'J.M. Jacquard" (public domain), cut out in black and white on Trama\'s purple. '
           'MIT OR Apache-2.0 (assets/app-icon/README.md). -->')

if __name__ == '__main__':
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    art = Image.open(sys.argv[1]).convert('RGB')
    out = Path(sys.argv[2])
    (out / 'trama.svg').write_text(master(art, COMMENT), encoding='utf-8')
    (out / 'trama-linux.svg').write_text(linux(art, COMMENT), encoding='utf-8')
