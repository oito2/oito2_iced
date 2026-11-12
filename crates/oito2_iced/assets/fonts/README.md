# Fonts

Inter 4.001 (git-9221beed3), licensed under the SIL Open Font License 1.1 (`OFL.txt`).

The eight files are static instances of `InterVariable.ttf` and
`InterVariable-Italic.ttf` from `oito2_ui` (`packages/oito2_ui/assets/fonts/inter`),
made with fontTools 4.68 by `tools/fonts/make_instances.py`:

```sh
python3 tools/fonts/make_instances.py InterVariable.ttf InterVariable-Italic.ttf crates/oito2_iced/assets/fonts
```

| Weight | Upright | Italic |
|---|---|---|
| 400 | `Inter-Regular.ttf` | `Inter-Italic.ttf` |
| 500 | `Inter-Medium.ttf` | `Inter-MediumItalic.ttf` |
| 600 | `Inter-SemiBold.ttf` | `Inter-SemiBoldItalic.ttf` |
| 700 | `Inter-Bold.ttf` | `Inter-BoldItalic.ttf` |

Each instance pins `wght` to its weight and `opsz` to 14, belongs to the family
`Inter` with the subfamily of its weight and style, and has the OS/2 weight
class of its weight.

Static instances are embedded because the text stack matches a font face by
its exact weight: a single variable face is only chosen at its default weight
(400), and the other weights fall back to a system font.
