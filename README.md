<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/com.localphoto.app.png" width="128" height="128" alt="Local Photo icon: a white camera iris on a fuchsia tile">
</p>

<h1 align="center">Local Photo</h1>

<p align="center">
  <b>Biblioteca de fotos e revelação RAW não destrutiva, open-source e clean-room, em Rust puro.</b><br>
  Catálogo, revelação (RAW/JPEG/HEIF), denoise, máscaras por IA, faces, HDR/panorama e export,<br>
  num app nativo escrito inteiramente em Rust. Offline, e seu.
</p>

<p align="center">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-d97706?style=flat-square">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-b45309?style=flat-square">
  <img alt="No account, no telemetry" src="https://img.shields.io/badge/no%20account-no%20telemetry-b45309?style=flat-square">
  <br>
</p>

<p align="center">
  <a href="https://github.com/Anon5T4R/Local-Photo"><b>Local Photo no GitHub</b></a> ·
  <a href="https://github.com/Anon5T4R/">Suíte Local</a> ·
  <a href="https://github.com/Anon5T4R">Anon5T4R</a>
</p>

<br>

---

## Get it

Os instaladores ficam anexados a cada [release do GitHub](https://github.com/Anon5T4R/Local-Photo/releases)
deste repositório. Sem loja, sem conta e sem telemetria: o app nunca fala com um servidor nosso, e o
único acesso à rede é a checagem de atualização contra o endpoint público de latest release do
GitHub (desligável nas preferências).

| Plataforma | Arquivos |
|---|---|
| Windows 10+ x64 | `localphoto-<ver>-windows-<arch>.msi` (per-machine, instala em `Program Files\Local Photo`) e `localphoto-<ver>-windows-<arch>-portable.zip` |
| Linux x86_64 | `.AppImage`, `.deb`, `.rpm` e `.tar.gz` (árvore FHS, `packaging/linux/package.sh`) |
| Arch Linux | `.pkg.tar.zst` do job Arch e `packaging/arch/PKGBUILD` |
| macOS, FreeBSD | pelos scripts de `packaging/` (não são buildados no CI deste fork) |

## Everything in the box

<h4>Biblioteca e revelação</h4>
Catálogo por pastas com journal próprio, miniaturas em cache, revelação não destrutiva por
"cópia virtual", presets e perfis de câmera, curvas, HSL/Color Mixer, máscaras (gradiente, radial,
pincel, por IA via SAM 3), denoise (modelos com endereço e SHA-256 pinados), correção de lente,
merge HDR/panorama, slideshow, export JPEG/TIFF/PNG/AVIF/JXL e fila em background.

<h4>Formatos RAW</h4>
CR2/CR3, NEF/NRW, ARW/SR2, RAF, ORF, RW2/RWL, PEF, SRW, DNG (inclusive lossy e tiles JPEG) e o
HEIF opcional (`--features heif`). O decodificador é nosso, em Rust puro, com corpus de referência
pinado por sha256 em `docs/raw-parity.md`.

<h4>Tudo é comando</h4>
UI, CLI (`localphoto-cli`), canal de controle JSON-lines e servidor MCP compartilham os mesmos ids
de comando: qualquer coisa que a UI faz, um agente também faz. Ver `docs/control-protocol.md` e
`docs/mcp.md`.

## Build

```sh
git clone https://github.com/Anon5T4R/Local-Photo
cd Local-Photo
cargo run --release -p localphoto            # o app desktop
cargo test --workspace                       # a suíte de testes
cargo xtask ci                               # gate completo (fmt, clippy, testes, camadas, assets)
```

As fontes CJK da UI e das legendas vêm do [craft-fonts](https://github.com/storytold/craft-fonts),
vendored em `assets/craft-fonts/` (o CI e os releases passam como `CRAFT_FONTS_DIR`). Sem ele tudo
compila e roda, mas chinês/japonês ficam sem glifos.

Os forks vendorizados ficam em `libs/`: `craft-libs` (com `crates/update` vindo do repo
`local-libs`, a fonte única da suíte), `egui` `6d518fc`, `winit` `a517b38`, `winprint.rs` `417ac3c`
e `rustls-rustcrypto`. `Cargo.lock` da raiz e o de `libs/craft-libs` são commitados e o CI recusa
qualquer dependência `git+`.

Mantenedores cortam releases empurrando uma tag `vX.Y.Z`; [`docs/releasing.md`](docs/releasing.md)
descreve o que o workflow builda e publica. Contribuidores e agentes começam por
[`AGENTS.md`](AGENTS.md), depois [`docs/`](docs/).

## License and credits

Local Photo é dual-licenciado sob [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), à sua escolha.
Copyright (c) 2026 Local Team e os contribuidores do Local Photo; a linha de copyright do upstream é
mantida em [`LICENSE-MIT`](LICENSE-MIT). Avisos obrigatórios em [NOTICE](NOTICE).

Fontes, ícones, imagens e outros assets mantêm suas próprias licenças abertas; cada um está listado
com autor, fonte e licença em [ATTRIBUTION.md](assets/ATTRIBUTION.md). Os releases também embutem as
fontes do [craft-fonts](https://github.com/storytold/craft-fonts/blob/main/ATTRIBUTION.md)
(SIL Open Font License 1.1).

Fork independente do lightcraft, mantido por Anon5T4R na suíte Local. Créditos do código original em
[NOTICE](NOTICE).

<p align="center">
  <sub>Suíte Local — apps locais, sem conta, sem telemetria.</sub>
</p>
