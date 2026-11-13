# SPDX-License-Identifier: GPL-3.0-only
# Copyright (C) 2026  OITO2
"""Writes static Inter instances from the Inter variable fonts.

Usage: python3 make_instances.py <InterVariable.ttf> <InterVariable-Italic.ttf> <output dir>

Each instance pins the weight axis to one weight and the optical size axis
to 14, and names the face in the family "OITO2 Inter" with the subfamily of
its weight and style, so an Inter installed on the system never takes its
place. The files keep the names Inter-<Subfamily>.ttf. Requires fontTools.
"""

import sys
from pathlib import Path

from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont

WEIGHTS = [("Regular", 400), ("Medium", 500), ("SemiBold", 600), ("Bold", 700)]
NAME_IDS = (1, 2, 3, 4, 6, 16, 17, 21, 22, 25)
FAMILY = "OITO2 Inter"
POSTSCRIPT_FAMILY = "OITO2Inter"


def instance(source: Path, weight: int, style: str, italic: bool, output: Path) -> None:
    font = instantiateVariableFont(TTFont(source), {"wght": weight, "opsz": 14})
    name = font["name"]
    version = name.getDebugName(5)

    subfamily = style if not italic else ("Italic" if style == "Regular" else f"{style} Italic")
    ribbi = style in ("Regular", "Bold")
    stem = subfamily.replace(" ", "")
    postscript = f"{POSTSCRIPT_FAMILY}-{stem}"

    for record in list(name.names):
        if record.nameID in NAME_IDS:
            name.removeNames(nameID=record.nameID)

    legacy_family = FAMILY if ribbi else f"{FAMILY} {style}"
    legacy_subfamily = {
        (True, False): style,
        (True, True): "Italic" if style == "Regular" else "Bold Italic",
        (False, False): "Regular",
        (False, True): "Italic",
    }[(ribbi, italic)]

    name.setName(legacy_family, 1, 3, 1, 0x409)
    name.setName(legacy_subfamily, 2, 3, 1, 0x409)
    name.setName(f"{version};{postscript}", 3, 3, 1, 0x409)
    name.setName(FAMILY if subfamily == "Regular" else f"{FAMILY} {subfamily}", 4, 3, 1, 0x409)
    name.setName(postscript, 6, 3, 1, 0x409)

    if not ribbi:
        name.setName(FAMILY, 16, 3, 1, 0x409)
        name.setName(subfamily, 17, 3, 1, 0x409)

    bold = style == "Bold"
    selection = font["OS/2"].fsSelection & ~0b1100001
    selection |= (0b1 if italic else 0) | (0b100000 if bold else 0)
    if not italic and not bold:
        selection |= 0b1000000
    font["OS/2"].fsSelection = selection
    font["OS/2"].usWeightClass = weight
    font["head"].macStyle = (1 if bold else 0) | (2 if italic else 0)

    font.save(output / f"Inter-{stem}.ttf")


def main() -> None:
    upright, italic, output = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
    output.mkdir(parents=True, exist_ok=True)

    for style, weight in WEIGHTS:
        instance(upright, weight, style, False, output)
        instance(italic, weight, style, True, output)


if __name__ == "__main__":
    main()
