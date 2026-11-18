# Fonts

JetBrains Mono 2.304, licensed under the SIL Open Font License 1.1 (`OFL.txt`).

The four files are static instances of `JetBrainsMono-Variable.ttf` and
`JetBrainsMono-Italic-Variable.ttf` from `oito2_ui`
(`packages/oito2_ui_syntax/assets/fonts/jetbrains_mono`), made with fontTools 4.68
by `tools/fonts/make_instances.py`:

```sh
python3 tools/fonts/make_instances.py --jetbrains-mono JetBrainsMono-Variable.ttf JetBrainsMono-Italic-Variable.ttf crates/oito2_iced_syntax/assets/fonts
```

| Weight | Upright | Italic |
|---|---|---|
| 400 | `JetBrainsMono-Regular.ttf` | `JetBrainsMono-Italic.ttf` |
| 700 | `JetBrainsMono-Bold.ttf` | `JetBrainsMono-BoldItalic.ttf` |

Each instance pins `wght` to its weight, belongs to the family
`OITO2 JetBrains Mono` (PostScript names `OITO2JetBrainsMono-<Subfamily>`) with
the subfamily of its weight and style, and has the OS/2 weight class of its
weight. The family has its own name so that a JetBrains Mono installed on the
system, often another version, never replaces the embedded faces.

Static instances are embedded because the text stack matches a font face by
its exact weight: a single variable face is only chosen at its default weight
(400), and the other weights fall back to a system font.
