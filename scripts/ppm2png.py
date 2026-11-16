#!/usr/bin/env python3
"""Convert a plain-text (P3) PPM, as written by this project, to a PNG.

Usage: python3 scripts/ppm2png.py out.ppm out.png
Standard library only.
"""
import struct
import sys
import zlib


def chunk(tag, data):
    body = tag + data
    return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)


def main(src, dst):
    with open(src, encoding="ascii") as f:
        toks = [t for line in f for t in line.split("#")[0].split()]
    if not toks or toks[0] != "P3":
        sys.exit("expected a plain-text P3 PPM")
    w, h, maxv = (int(t) for t in toks[1:4])
    vals = [int(t) for t in toks[4:]]
    if len(vals) != w * h * 3:
        sys.exit(f"expected {w * h * 3} channel values for {w}x{h}, found {len(vals)}")

    raw = bytearray()
    for y in range(h):
        raw.append(0)  # PNG filter type: none
        row = vals[y * w * 3:(y + 1) * w * 3]
        raw.extend(max(0, min(255, v * 255 // maxv)) for v in row)

    png = (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(bytes(raw), 9))
        + chunk(b"IEND", b"")
    )
    with open(dst, "wb") as f:
        f.write(png)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    main(sys.argv[1], sys.argv[2])
