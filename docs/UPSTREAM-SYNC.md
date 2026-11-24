# Upstream sync

The Local suite's port/rebrand: `main` is ours, the upstream is a source of candidate changes and
nothing enters without review. General rules in `docs/planos/licoes-de-porta.md` (dev-notes).

## Base

- Upstream: `storytold/lightcraft`, commit **bb08a69** (`0.14.0`), no tag on the commit — the tree
  was copied from that commit's `git archive` (with no history).
- Fork: `Styx-Studios/Local-Photo`, own version `0.1.x` (a new numbering avoids a false auto-update to
  the upstream).
- Nothing is read from `C:\Dev\storytold` in build, test or release; the folder is only a local
  reference for the `sync-upstream` skill.

## Vendored (outside the normal sync)

| Path | Origin | Rev |
|---|---|---|
| `libs/craft-libs` | `storytold/craft-libs` | `1e6f1278` |
| `libs/craft-libs/crates/update` | **`Styx-Studios/local-libs`** (the suite's single source) | see the repo's `HASHES.update` |
| `libs/egui` | `storytold/egui` | `6d518fc` |
| `libs/winit` | `storytold/winit` | `a517b38` |
| `libs/winprint.rs` | `storytold/winprint.rs` | `417ac3c` (craft-libs also pins `5f80b9b`; both crates resolve by path) |
| `libs/rustls-rustcrypto` | the suite's fork (100% Rust TLS provider) | `0.0.2-alpha` |
| `assets/craft-fonts` | `storytold/craft-fonts` (build input `CRAFT_FONTS_DIR`) | `b076417` |

`craft-update` is a **conscious permanent fork** and stays outside the sync: bugfixes go straight
into the source (`local-libs`) and are replicated with `sync.ps1 -Mode push` (hash guard; the copy
here is a mirror). Never edit `libs/craft-libs/crates/update` by hand.

## Rebrand divergences (what is ours)

- Identity: crates `localphoto-*`, binaries `localphoto`/`localphoto-cli`, app id
  `com.localphoto.app` (desktop entry, hicolor Icon, Wayland app id, Info.plist, pacman),
  MSI ProgIds/UpgradeCode, registry keys (`Software\Local Photo` +
  `Software\Anon5T4R\LocalPhoto\InstallDir`, App Paths), configuration folder `LocalPhoto`.
- Own CI: no `craft-common-actions`, no sonarcloud, no nix/flake, no macOS/FreeBSD/windows-arm64
  jobs. Release by **tag** (the upstream uses the `release` branch + draft).
- `libs/` and `assets/craft-fonts/` vendored (the upstream fetches both by git in CI).
- `cargo xtask fmt` (scope = workspace members) in place of `cargo fmt --all`: the vendored paths
  keep their origin's style.
- Community links: the menus/About point only to this repository (the upstream had its own site and
  Discord).
- Own icon (v0.1.2): a white camera iris on a fuchsia tile, SVG drawn in this repo (the upstream
  used an engraved lynx, inherited art that left entirely — hicolor, `.ico`, `.icns`,
  `localphoto-1024.png`, DMG background). Generation: `packaging/icons.sh` + `cargo xtask ico` +
  PNG-chunk `.icns` packer (no macOS).
- MSI (v0.1.2): a **desktop** shortcut besides the Start menu (own component, non-advertised
  shortcut, key path in the `Software\Local Photo\Shortcuts` registry), local-paint's standard.
- `packaging/windows/package.ps1` builds the portable zip with `ZipArchive` and `/` bars (PS 5.1's
  `Compress-Archive` writes `\`, which the updater refuses).
- `crates/ui-egui/src/render.rs` (`request_needed`): a new `clippy::needless_bool` from stable 1.100
  rejected the upstream's `if … { return false; } true`. The condition was negated (the lint's own
  suggestion), with no behavior change — a minimal local divergence until the upstream catches up.
- `.github/`, README, AGENTS/CLAUDE and `docs/releasing.md` rewritten for the suite.

## Sync log

The base stays `bb08a69`: only chosen points enter (skill `sync-upstream`), not a bump.

| Date | Target (lightcraft) | Taken | Relevant discards |
|---|---|---|---|
| 2026-11-21 | the day's `origin/main` | series #2589 `de9b0a89` `7c74ae0a` `a31a4367` `5b8b9b52` → `8682269` (duplicate by content in the Lightroom catalog); `8ffe73cc` → `89fbbc7` (X-S20 lens, X-S10/X-S20/X-M5 sensors) | `2d905e0a` `89d11ab1` (.lrcat ZIP backup: we don't have it), `5c598189` `d7372407` `e689453e` `19985a3f` `690f6cfb` (catalog log: we don't have it), `6c3083eb` `5a242898` (depend on `craft-raw`), `cb91d69e` (hot pixel: we don't have it), `a57bd69d` (minor perf, depends on the new job) |

## Known pending items/gaps of this port

- minisign signing of assets: fail-open until the `MINISIGN_KEY`/`MINISIGN_PASSPHRASE` secrets
  exist in this repo (see `docs/releasing.md`).
- macOS: `packaging/macos/dmg/dmg-layout.DS_Store` still carries the upstream name in the Finder
  layout (regenerating requires macOS); the bundle itself is already `Local Photo.app`.
- Tests run in CI on Linux only (as in the upstream); on Windows the job compiles `cargo test
  --no-run` — the UI tests render through wgpu and the runner has no GPU. Three upstream tests did
  not pass on Windows and were fixed/limited here: two compared paths built with `join("a/b")`
  (mixed separator) and one expects hard-link detection by a different name, which on Windows would
  depend on the file index (unstable metadata) — the *admission* assertion became `#[cfg(unix)]`
  and the *write-time* one (the path that showed up) still holds on both.
- `docs/images/` and `docs/showcase/` are captures of the upstream app (old title in the pixels);
  regenerate once the look stabilizes.
- **Load flake (not suppressed):**
  `tests_segment::background_requests_never_wait_and_failures_come_back_as_messages`
  measures the time to enqueue requests to the model (< 1 s) and burst once (1.8 s) on local
  Windows with the whole suite in parallel; it passes isolated and in the crate's binary (1301
  tests). Code untouched here — if it flakes in CI, look into the model's I/O contention before
  touching the limit.
- Local tests require `CRAFT_FONTS_DIR=assets/craft-fonts` (the accessibility test draws "▸",
  which only exists in the craft-fonts set; CI already sets the variable).
