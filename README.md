<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/com.localphoto.app.png" width="96" height="96" alt="Local Photo icon: a white camera iris on a fuchsia tile">
</p>

<h1 align="center">Local Photo</h1>

<p align="center">
  A photo library and non-destructive RAW developer for Windows and Linux. Written in Rust, offline, no account.<br>
  <a href="https://github.com/Anon5T4R/Local-Photo/releases/latest">Download</a> ·
  <a href="ROADMAP.md">Roadmap and status</a> ·
  <a href="#where-it-comes-from">Where it comes from</a> ·
  <a href="#how-it-is-made">How it is made</a>
</p>

<!-- screenshot: main window with a photo open in Develop, filmstrip and right panel; dark and light -->

The library is a folder you choose, and the original files are never overwritten: the edits live in
the catalog, or in an XMP sidecar next to the file. The benchmark for behavior is Lightroom Classic.

## What it does

- **Library.** Import by reference, copy or move; albums and smart albums, stacks, virtual copies and
  versions; ratings, flags, color labels, keywords and a filter bar. A People view from the face
  names already in the files and from faces it finds when you install the detection model, and an
  import of a Lightroom Classic catalog (`.lrcat`).
- **Develop.** White balance, tone, curves, HSL/color mixer, color grading, texture, clarity, dehaze,
  sharpening, noise reduction and grain; optics (corrections from the lens data the camera records,
  chromatic aberration, defringe, vignette); crop, straighten, Upright and transforms; masks (linear,
  radial, brush, and Object/Describe with SAM 3); Remove (heal and clone) and Red Eye.
- **Merge.** HDR, panorama and HDR panorama with a live preview, and HDR for every bracket set of a
  selection in one go.
- **Culling.** Assisted Culling scores focus and clipping and groups similar shots.
- **Output.** Export as JPEG, PNG, TIFF, WebP, PSD, AVIF or JPEG XL, as the untouched original with a
  sidecar, or as a DNG; a print layout and a contact sheet written as PDF; a slideshow, exported as
  PDF, as JPEG frames or as AV1 video.
- **Library health.** Find missing photos and relink them; back the library up and restore it.
- **The same commands everywhere.** The interface, `localphoto-cli`, the JSON-lines control channel and
  the MCP server dispatch the same command ids, so an agent can do what a person does
  ([docs/control-protocol.md](docs/control-protocol.md), [docs/mcp.md](docs/mcp.md)).
- **Interchange.** XMP sidecars with Adobe `crs:` interop for metadata and develop settings, and import
  or export of presets, profiles, keywords and point curves ([docs/xmp-interop.md](docs/xmp-interop.md)).

What works, what is weak and what is planned is in the [roadmap](ROADMAP.md).

## Formats

| | |
|---|---|
| Opens | Camera raw: Canon CR2 and CR3, Nikon NEF and NRW, Sony ARW, SR2 and SRF, Fujifilm RAF, Olympus ORF, Panasonic RW2 and RWL, Pentax PEF, Samsung SRW, DNG (including Apple ProRAW, lossy and JPEG XL tiles), and CRW, MRW, IIQ, X3F, KDC, MOS, ERF, 3FR, FFF, MEF and `.raw`. Standard images: JPEG, PNG, TIFF, WebP, PSD and PSB, JPEG XL, GIF, BMP. HEIC/HEIF in builds compiled with the `heif` feature, which the releases are. |
| Saves | The library folder: `catalog.snap` with `catalog.log`, an append-only journal of operations, a lock file, and `Originals/` for imported copies. XMP sidecars for the metadata and the develop settings, when you save them. |
| Exports | JPEG, PNG, TIFF, WebP, PSD, AVIF and JPEG XL with a size, sharpening and metadata policy; the untouched original plus its XMP; a DNG of the raw data, or a rendered linear DNG. Output in sRGB, Display P3, Adobe RGB, ProPhoto RGB or Rec. 2020 with an embedded ICC profile. |
| Imports | A Lightroom Classic catalog, XMP sidecars, presets and profiles, keywords and point-curve presets. |

A file extension in that list is not a promise of sensor decoding: some bodies open from the camera's
embedded preview only, and a few rare variants are refused. [docs/raw-parity.md](docs/raw-parity.md)
measures it camera by camera.

## Network and privacy

- Photos and edits never leave the computer.
- On start the app asks GitHub whether a newer release exists, and downloads it only if you accept.
  There is no setting to turn the check off yet.
- Everything else that uses the network is a download you ask for: model files (the face detection
  model, and SAM 3 from a mirror you configure) and the GeoNames place names used to turn GPS
  coordinates into place names. A model's manifest pins a size and, where it is known, a SHA-256 that
  the download is checked against; the place names come from `download.geonames.org`.
- No account, analytics or crash reporting.

Security notes, all checkable in this repository:

- The app's crates and binaries forbid `unsafe` code, and none contains an `unsafe` block; the one
  exception is `crates/sysmem`, with a single FFI call (macOS memory-pressure relief).
- The raw decoders, the develop pipeline and the encoders are Rust; there is no LibRaw or dcraw.
- TLS for the update check and the downloads is rustls with the suite's RustCrypto provider
  (`libs/rustls-rustcrypto`); no OpenSSL and no aws-lc.
- Releases are built by GitHub Actions from a tagged commit with locked dependencies; `cargo-deny`
  checks advisories, licenses and bans, and no dependency points at a git URL.

## Download

Windows (x64): `.msi` per-machine installer or portable `.zip`. Linux (x86_64): `.AppImage`, `.deb`,
`.rpm` or `.tar.gz`, and Arch Linux gets a `.pkg.tar.zst` built from the same tarball. All in
[Releases](https://github.com/Anon5T4R/Local-Photo/releases). macOS and FreeBSD have packaging scripts
in the repository, but no published build.

Every release has a `SHA256SUMS.txt`. When the signing key is configured, each asset also gets a
[minisign](https://jedisct1.github.io/minisign/) signature from the suite's key; a release is published
unsigned rather than with a signature that does not match.

```sh
minisign -Vm <file> -P RWTqN+hTTCFbDCGCA1cEJRLp2FSluxIUiJfqdoAYwkwNNVH5MTYH8qSN
```

The public key is also in [packaging/minisign.pub](packaging/minisign.pub). It is the same key for every
Local app.

## Where it comes from

Local Photo is a fork of [lightcraft](https://github.com/storytold/lightcraft) by
[ArtCraft](https://getartcraft.com), MIT or Apache-2.0. The tree was copied from their commit
`bb08a69` (version 0.14.0), without history. The raw decoders, the develop pipeline, the catalog and
the interface are their work, and so is the decision to write the whole app in Rust with egui.

**Why a fork.** We wanted a set of desktop apps that look and behave the same, update the same way,
and follow our own priorities: Windows and Linux first, files that stay local, AI only with your own
model, and agents able to drive every app. A fork lets us decide that without asking the upstream to
change direction.

We also wanted "written in Rust" to hold for everything the app runs. The upstream updater builds its
TLS on `ring`, which is partly C and assembly; here the update check and the downloads use the
pure-Rust RustCrypto provider.

**What changed here:**

- Name, icon and app identifiers.
- The shell shared by the Local apps: Automatic/Light/Dark appearance, the command palette, the
  settings and the update dialog.
- Our own updater (`craft-update`, a permanent fork kept outside the upstream sync) with its signature
  check, and our own release pipeline (Windows and Linux packages, Arch).
- TLS moved to `libs/rustls-rustcrypto`.
- The vendored trees (`libs/`, `assets/craft-fonts`) live in this repository; the upstream's own CI is
  replaced by ours, and macOS, FreeBSD and Windows ARM64 are not built in CI.
- Our own version numbering, starting at 0.1, so an installation cannot mistake this fork for the
  upstream.

**What we still take from upstream:** bug fixes that matter to us (crashes, data loss, security, file
compatibility), reviewed one by one and adapted. New features and UI changes are not merged
automatically. Each sync is recorded in [docs/UPSTREAM-SYNC.md](docs/UPSTREAM-SYNC.md).

## How the app is built

One Cargo workspace, with the library in layers: a crate may depend only on lower layers, and nothing
below the interface knows about egui. `cargo xtask layers` enforces it.

| Layer | Examples | Role |
|---|---|---|
| Foundation | `geom`, `color`, `raster`, `tiff`, `fetch` | geometry and warps, color spaces, float image buffers, TIFF, pinned downloads |
| Decode and edit | `raw`, `codecs`, `meta`, `develop`, `denoise`, `faces`, `heif` | raw decoders and demosaic, standard codecs and encoders, EXIF/XMP/IPTC, the edit model and its controls |
| Pipeline | `pipeline`, `gpu` | the scene-referred develop pipeline, and its wgpu twin |
| Library | `catalog`, `preview`, `merge`, `segment` | catalog and journal, preview cache, HDR and panorama merge, SAM 3 segmentation |
| Engine | `engine` | session, command registry, import, export, background jobs |
| Front ends | `ui-egui`, `mcp`, `apps/localphoto`, `apps/localphoto-cli` | the interface, the MCP server, the desktop app and the CLI |

The develop pipeline exists twice against one specification: a CPU reference and the same stages in
WGSL on wgpu. The GPU path is checked against the CPU one (within 1/255), and an unsupported adapter or
a GPU error falls back to the CPU ([docs/gpu-pipeline.md](docs/gpu-pipeline.md)). More:
[docs/architecture.md](docs/architecture.md).

## Build from source

Rust 1.95 or newer.

```sh
git clone https://github.com/Anon5T4R/Local-Photo
cd Local-Photo
cargo run --release -p localphoto   # the app
cargo xtask ci                      # fmt, clippy, tests, layers, assets, cargo-deny
```

The CJK fonts the interface needs are vendored in `assets/craft-fonts` and wired in by
`.cargo/config.toml`, so a plain build has them. HEIC/HEIF support is not in the default build (HEVC
patents), but the releases include it: `cargo run --release -p localphoto --features heif` builds the
same way.

## Keyboard shortcuts

`G` grid, `D` detail, `E` edit panel, `M` masking, `C` crop, `H` remove, `1`–`5` star rating, `P`
pick, `X` reject, `U` clear the flag, `F` full screen, `Z` zoom. With the system modifier:
`Ctrl+Shift+I` import, `Ctrl+P` print, `Ctrl+O` open a library, `Ctrl+,` settings, `Ctrl+/` the full
list, `F1` help. Every shortcut can be changed in Help → Keyboard Shortcuts.

## How it is made

Local Photo follows the same rules as every Local app: local files and no account, Rust only (including
the interface and the cryptography), one self-contained repository per app, forks with selective sync,
governed dependencies, releases built from a tag, optional AI with your own model, and MCP so
agents can use the app. Most of the code is written by AI coding agents (Claude Code and OpenCode) from
written plans, reviewed by the maintainer and held to the same checks and CI. The reasons for each
decision:
[How we make the Local apps](https://github.com/Styx-Studios/.github/blob/main/profile/README.md#how-we-make-the-local-apps).

## AI

No AI feature needs an account, and no weights ship with the app. Each piece is optional and separate:

- **Masks.** Object selects by clicking, and Describe by naming (`sky`, `the red car`): SAM 3, run in
  Rust with candle on the CPU on Windows and Linux. The checkpoint (about 3.4 GB) is not bundled and is
  under Meta's SAM License; the official repository is gated, so you get the files yourself or point
  the app at a mirror. Without them, every other mask and the rest of the app work.
- **AI Denoise.** A model you install from a file (Settings → AI Denoise). The default build offers no
  download; a pinned RawNIND download exists only in builds compiled with the `rawnind-model` feature,
  and those weights are GPL-3.0.
- **Faces.** Detection (YuNet) is a small download; recognition models are large and are yours to
  install. The face names already in the files need neither.

Details: [docs/ai-masks.md](docs/ai-masks.md), [docs/denoise.md](docs/denoise.md),
[docs/faces.md](docs/faces.md).

## License and credits

MIT or Apache-2.0, at your option ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)). The
upstream copyright line stays in the MIT license, and the third-party notices (fonts, models, showcase
photographs, camera color data) are in [NOTICE](NOTICE) and
[assets/ATTRIBUTION.md](assets/ATTRIBUTION.md). Other Local apps:
[the full list](https://github.com/Styx-Studios).
