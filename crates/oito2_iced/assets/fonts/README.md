# Fonts

Inter 4.001 (git-9221beed3), licensed under the SIL Open Font License 1.1 (`OFL.txt`).

The four files are static instances of `InterVariable.ttf` from `oito2_ui`
(`packages/oito2_ui/assets/fonts/inter`), made with fontTools 4.68:

```sh
fonttools varLib.instancer -o Inter-<Name>.ttf InterVariable.ttf wght=<weight> opsz=14
```

| File | `wght` | `opsz` |
|---|---|---|
| `Inter-Regular.ttf` | 400 | 14 |
| `Inter-Medium.ttf` | 500 | 14 |
| `Inter-SemiBold.ttf` | 600 | 14 |
| `Inter-Bold.ttf` | 700 | 14 |

The name tables were then set to the family `Inter` with the subfamily of each
weight (PostScript names `Inter-Regular`, `Inter-Medium`, `Inter-SemiBold`,
`Inter-Bold`), and the OS/2 `usWeightClass` matches the `wght` value.

Static instances are embedded because the text stack matches a font face by
its exact weight: a single variable face is only chosen at its default weight
(400), and the other weights fall back to a system font.
