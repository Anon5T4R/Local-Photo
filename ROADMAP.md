# ROADMAP — Local Photo

Fork of lightcraft, rebranded for the Local suite. Pure Rust + egui, desktop only: Windows (MSI and
portable) and Linux (AppImage, deb, rpm, pacman, tar.gz). Library, RAW development, masks, HDR/panorama
merge, printing and export, with its own catalog and Lightroom catalog import. How it is built:
`docs/architecture.md`. What comes from the upstream: `docs/UPSTREAM-SYNC.md`.

**Stage: alpha.** The central flows work end to end: import, pick, develop, save and reopen, apply
looks in batch and export. What is below is the path to "ready for real work".

## What is missing (by area, in no order of difficulty)

- **RAW.** Coverage per model is measured in `docs/raw-parity.md` (with the corpus pinned in
  `docs/raw/`). Compressed ORF and HE NEF open as a preview only. Rare CR3 variants too. Many
  models decode, but without verification on a real file.
- **Camera color.** Measured calibration for the popular models is missing: most use an estimated
  matrix, and only a few bodies have their own profile. The rendering was tuned by eye, without a
  fidelity battery against a reference.
- **Lenses.** No lens-profile base of our own. Today the correction comes from the data the camera
  records (Sony, Fujifilm, Nikon: docs `*-lens-corrections.md`).
- **AI masks.** Subject, sky and people are heuristics. The model and how to distribute it follow
  the suite's local-AI standard ("Install from file…", like AI Denoise).
- **Remove, Heal and Enhance.** Leaner than Lightroom's.
- **Output modules.** Map, book, publish and tethering do not exist. Slideshow and print do.
- **Video.** It does not import or edit video.
- **Localization.** Complete English. The other catalogs (`crates/ui-egui/locales`) are partial.
  Hindi, Arabic and Korean do not have letter shapes resolved (shaping/RTL, Hangul face).

## References

`docs/architecture.md` · `docs/raw-parity.md` · `docs/control-protocol.md` · `docs/mcp.md` ·
`docs/denoise.md` · `docs/lightroom-catalog-import.md` · `docs/localization.md` ·
`docs/UPSTREAM-SYNC.md`
