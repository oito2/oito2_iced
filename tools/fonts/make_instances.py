# SPDX-License-Identifier: GPL-3.0-only
# Copyright (C) 2026  OITO2
"""Writes static instances of the Inter or JetBrains Mono variable fonts.

Usage: python3 make_instances.py [--jetbrains-mono] <Variable.ttf> <Variable-Italic.ttf> <output dir>

Without a flag the sources are the Inter variable fonts: each instance pins
the weight axis to Regular, Medium, SemiBold or Bold and the optical size
axis to 14, in the family "OITO2 Inter", in files Inter-<Subfamily>.ttf.
With --jetbrains-mono the sources are the JetBrains Mono variable fonts:
each instance pins the weight axis to Regular or Bold, in the family
"OITO2 JetBrains Mono", in files JetBrainsMono-<Subfamily>.ttf.

Each face is named with the subfamily of its weight and style in its own
family, so a copy of the font installed on the system never takes its
place. Requires fontTools.
"""

import sys
from pathlib import Path

from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont

NAME_IDS = (1, 2, 3, 4, 6, 16, 17, 21, 22, 25)

INTER = {
    "weights": [("Regular", 400), ("Medium", 500), ("SemiBold", 600), ("Bold", 700)],
    "axes": {"opsz": 14},
    "family": "OITO2 Inter",
    "postscript": "OITO2Inter",
    "file": "Inter",
}

JETBRAINS_MONO = {
    "weights": [("Regular", 400), ("Bold", 700)],
    "axes": {},
    "family": "OITO2 JetBrains Mono",
    "postscript": "OITO2JetBrainsMono",
    "file": "JetBrainsMono",
}


def instance(
    spec: dict, source: Path, weight: int, style: str, italic: bool, output: Path
) -> None:
    family = spec["family"]
    postscript_family = spec["postscript"]
    font = instantiateVariableFont(TTFont(source), {"wght": weight, **spec["axes"]})
    name = font["name"]
    version = name.getDebugName(5)

    subfamily = style if not italic else ("Italic" if style == "Regular" else f"{style} Italic")
    ribbi = style in ("Regular", "Bold")
    stem = subfamily.replace(" ", "")
    postscript = f"{postscript_family}-{stem}"

    for record in list(name.names):
        if record.nameID in NAME_IDS:
            name.removeNames(nameID=record.nameID)

    legacy_family = family if ribbi else f"{family} {style}"
    legacy_subfamily = {
        (True, False): style,
        (True, True): "Italic" if style == "Regular" else "Bold Italic",
        (False, False): "Regular",
        (False, True): "Italic",
    }[(ribbi, italic)]

    name.setName(legacy_family, 1, 3, 1, 0x409)
    name.setName(legacy_subfamily, 2, 3, 1, 0x409)
    name.setName(f"{version};{postscript}", 3, 3, 1, 0x409)
    name.setName(family if subfamily == "Regular" else f"{family} {subfamily}", 4, 3, 1, 0x409)
    name.setName(postscript, 6, 3, 1, 0x409)

    if not ribbi:
        name.setName(family, 16, 3, 1, 0x409)
        name.setName(subfamily, 17, 3, 1, 0x409)

    bold = style == "Bold"
    selection = font["OS/2"].fsSelection & ~0b1100001
    selection |= (0b1 if italic else 0) | (0b100000 if bold else 0)
    if not italic and not bold:
        selection |= 0b1000000
    font["OS/2"].fsSelection = selection
    font["OS/2"].usWeightClass = weight
    font["head"].macStyle = (1 if bold else 0) | (2 if italic else 0)

    font.save(output / f"{spec['file']}-{stem}.ttf")


def main() -> None:
    args = sys.argv[1:]
    spec = INTER
    if args and args[0] == "--jetbrains-mono":
        spec = JETBRAINS_MONO
        args = args[1:]
    upright, italic, output = Path(args[0]), Path(args[1]), Path(args[2])
    output.mkdir(parents=True, exist_ok=True)

    for style, weight in spec["weights"]:
        instance(spec, upright, weight, style, False, output)
        instance(spec, italic, weight, style, True, output)


if __name__ == "__main__":
    main()
