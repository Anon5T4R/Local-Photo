# AGENTS.md — Local Photo

Fork independente do lightcraft (ArtCraft), rebrandado para a suíte Local do João.
Rust puro + egui. Sem conta, sem telemetria, sem dependência de servidores de terceiros.
Biblioteca de fotos + revelação RAW não destrutiva; o Lightroom é a referência de comportamento.

## Regras
- **Dependências:** toda dep que entra/sai/atualiza exige `Cargo.lock` commitado +
  linha no inventário do repo `Anon5T4R/local-deps` (regra completa em
  `local-deps/WORKFLOW.md`). PR sem isso não mergea.
- **Sem fontes git:** nenhuma dependência pode apontar para `git+` (o CI valida; vale para
  `Cargo.lock` da raiz e o de `libs/craft-libs`). Tudo vem do crates.io (pinado pelo lock) ou de
  `libs/` (vendored).
- **Commits:** português, sem Co-Authored-By.
- **Release:** tag `vX.Y.Z` → o Actions builda Windows + Linux + Arch e publica. Mínimo local.
- **Disco local:** nunca deixe `target/` para trás — `cargo clean` após uso local;
  builds pesados são do CI.

## Estrutura
- `crates/` — engine e módulos (`localphoto-*`), em camadas L0–L6 (`cargo xtask layers`)
- `apps/localphoto` — app desktop + control channel; `apps/localphoto-cli` — CLI/MCP;
  `apps/localphoto-web` — wasm
- `libs/` — forks vendored: `craft-libs` (base 1e6f1278; `crates/update` vem do `local-libs`),
  `egui` 6d518fc, `winit` a517b38, `winprint.rs` 417ac3c, `rustls-rustcrypto`
- `assets/craft-fonts/` — o craft-fonts inteiro vendored (build input `CRAFT_FONTS_DIR`)
- `packaging/` — Windows (WiX/MSI), Linux (AppImage/deb/rpm/tar), Arch (PKGBUILD), macOS, FreeBSD, web
- `docs/` — arquitetura, formatos, paridade, release (`docs/releasing.md`)
- `xtask/` — tarefas de build/CI (`cargo xtask ci`, `layers`, `assets`, `parity`, `wasm`, `web`)

## Agent-native (MCP)
`crates/mcp` (servidor stdio + headless/`Remote` do control channel loopback) e
`localphoto-cli mcp` (headless ou `--connect [ADDR]`). A crate compartilhada da suíte é
**`craft-mcp`** (fonte `local-libs`, espelho em `libs/craft-libs/crates/mcp`), usada no `--connect`
(porta nua = loopback; host público recusado). `run --script` aceita JSON Lines ou um array de
passos; `commands --search TEXT` filtra o catálogo. **Dívida**: as flags
`--automation-read-root`/`--automation-write-root` ainda não existem neste app. Docs: `docs/mcp.md`.

## Build
```
cargo build --locked          # debug
CRAFT_FONTS_DIR=assets/craft-fonts cargo test --locked   # testes (o set completo de fontes,
                                                         # como no CI; sem ele os testes de
                                                         # glifos/acessibilidade pulam ou falham)
cargo xtask ci                # gate completo (fmt, clippy, testes, camadas, assets, wasm)
packaging/windows/package.ps1 # MSI + portable (Windows)
packaging/linux/package.sh    # AppImage/deb/rpm/tar (Linux)
```

Fontes CJK: o craft-fonts vendored em `assets/craft-fonts` é o build input
(`CRAFT_FONTS_DIR=assets/craft-fonts cargo run -p localphoto`). Sem ele tudo compila e roda, mas
chinês/japonês ficam sem glifos; os releases sempre buildam com ele.

Regras técnicas do código (never-crash, camadas, tudo é comando, wasm, testes):
`AGENTS.md` do upstream equivale a `docs/contributing.md` quando existir; as regras vivas são:
nada de `unsafe` fora de `crates/sysmem`, nada de `unwrap`/`panic!` em código de produção,
toda entrada (arquivo, comando, MCP, preferências) é hostil até prova em contrário.

## Upstream e a pasta `C:\Dev\storytold`

Menções a `storytold/*` em código, comentários, docs e atribuições (NOTICE/ATTRIBUTION) são
referência histórica e obrigação de licença. **Nada** é lido de `C:\Dev\storytold` em build, teste
ou release: tudo vem do próprio repo (crates.io pinado pelo lock + `libs/`/`assets/craft-fonts`
vendored). Essa pasta existe somente como referência local para a skill `sync-upstream`.

## Assets: ícones, imagens, fontes
- Nenhum asset de produto proprietário (Adobe e afins). Só trabalho próprio, domínio público/CC0,
  CC-BY com atribuição, OFL ou permissivo (MIT/Apache).
- Todo asset tem linha em `assets/ATTRIBUTION.md` no mesmo commit (`cargo xtask assets` valida;
  `libs/` é árvore vendored e carrega a atribuição de origem).
- Fontes fora do Inter embutido vivem no craft-fonts (vendored em `assets/craft-fonts`).
