# Local Photo app icon

A **camera iris**: six blades around a hexagonal opening, on a full-bleed gradient tile - the mark
that tells Local Photo apart from the rest of the suite (a palette for paint, a bezier for vector, a
document for PDF...). It says "photograph" without repeating the framed-picture glyph of the image
viewer.

**Style:** flat shapes on a full-bleed gradient tile, the same language as the rest of the Local
suite's icons: a rounded square inset 32 units of 512 (`rx=100`), a white sheen over the top half,
one white glyph, and the "Taylor" signature at the tile's foot.

## Palette

| Colour | Hex | Used for |
|---|---|---|
| Fuchsia | `#e879f9` → `#a21caf` | gradient of the tile (top-left to bottom-right) |
| White | `#ffffff` | the iris blades (the glyph) |
| White at 16% | `#ffffff` | the sheen over the top half |
| White at 85% | `#ffffff` | the signature "Taylor" at the tile's foot |

The tile colour was picked from the suite's free hues: amber (paint), teal (vector), emerald
(sheets), orange (slides), blue (office, image), red-orange (PDF), indigo (keys), purple (utils).

## Geometry

`viewBox="0 0 512 512"`. The glyph is drawn with a mask so the tile gradient shows through the
opening and the blade cuts: a circle of radius 155 at (256, 252), a hexagon of radius 61 rotated
15°, and each hexagon edge continued past its end vertex to the outer ring (11.5-unit cuts). The
six blades are therefore exactly the ring minus the cuts, and composite over any gradient. The ring
is 310 units across, 69 % of the 448-unit tile - the same visual weight as the palette (paint) and
the bezier (vector).

macOS files (`.icns`, `localphoto-macos-512.png`) widen the viewBox to `-62 -62 636 636`, standing
in for Apple's 824/1024 body; Windows and Linux files use the full-bleed tile, so the iris reads at
16-48 px.

## Variants

`variants/localphoto-graphite.svg` is the same artwork on a graphite tile (`#64748b` → `#0f172a`) -
kept documented, not shipped: neutral tiles break the suite's one strong hue per app (amber, teal,
emerald, orange, blue, red-orange, indigo, purple, fuchsia). Use it only if a light/high-contrast
environment ever needs it; rendering it is `packaging/icons.sh` pointed at that file.

## Provenance

Original work for this fork, drawn as SVG in the repository (`localphoto.svg` is the master; the
older upstream lynx artwork this fork inherited is gone). Licence: `LICENSE.txt` (MIT OR
Apache-2.0, like the repo).

## Files

| File | What |
|---|---|
| `localphoto.svg` | canonical vector (about 1.9 KB) |
| `localphoto-small.svg` | the same master under a name the READMEs can link |
| `localphoto-1024.png` | 1024 px render (store listings, docs) |
| `localphoto-macos-512.png` | runtime window/Dock icon on macOS (embedded by `apps/localphoto/src/main.rs`) |
| `localphoto.icns` | macOS bundle icon (`CFBundleIconFile`; chunks `icp4`..`ic10`, PNG payloads) |
| `localphoto.ico` | Windows icon, 16-256 px, embedded in `localphoto.exe` by `apps/localphoto/build.rs` |
| `hicolor/<n>x<n>/apps/com.localphoto.app.png` | Linux icon theme, 16-512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/com.localphoto.app.svg` | Linux scalable icon (the small SVG) |

The app id is `com.localphoto.app`: the Wayland app id, the `.desktop` file
(`packaging/linux/com.localphoto.app.desktop`, `Icon=com.localphoto.app`) and the hicolor icon
name. To install on Linux, copy `hicolor/` into `/usr/share/icons/hicolor/` (or `~/.local/share/icons/hicolor/`)
and the `.desktop` file into `applications/`.

## Regenerate

Edit `localphoto.svg`, then run `packaging/icons.sh` (needs `resvg`; on Windows pass
`--use-font-file C:\Windows\Fonts\segoesc.ttf` so the signature renders with Segoe Script instead of
a fallback; `iconutil` on macOS writes the `.icns`). Every output is committed: the `.ico` is packed
by `cargo xtask ico`, and the `.icns` is an `icns` header plus PNG-payload chunks (`icp4` 16, `icp5`
32, `icp6` 64, `ic07` 128, `ic08` 256, `ic09` 512, `ic10` 1024; length prefix big-endian, including
the chunk header).
