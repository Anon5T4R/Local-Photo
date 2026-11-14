# Sincronização com o upstream

Porta/rebrand da suíte Local: o `main` é nosso, o upstream é fonte de mudanças candidatas e nada
entra sem revisão. Regras gerais em `docs/planos/licoes-de-porta.md` (dev-notes).

## Base

- Upstream: `storytold/lightcraft`, commit **bb08a69** (`0.14.0`), sem tag no commit — a árvore foi
  copiada do `git archive` desse commit (sem histórico).
- Fork: `Anon5T4R/Local-Photo`, versão própria `0.1.x` (numeração nova evita auto-update falso
  para o upstream).
- Nada é lido de `C:\Dev\storytold` em build, teste ou release; a pasta é só referência local para
  a skill `sync-upstream`.

## Vendorizado (fora do sync normal)

| Caminho | Origem | Rev |
|---|---|---|
| `libs/craft-libs` | `storytold/craft-libs` | `1e6f1278` |
| `libs/craft-libs/crates/update` | **`Anon5T4R/local-libs`** (fonte única da suíte) | ver `HASHES.update` do repo |
| `libs/egui` | `storytold/egui` | `6d518fc` |
| `libs/winit` | `storytold/winit` | `a517b38` |
| `libs/winprint.rs` | `storytold/winprint.rs` | `417ac3c` (o craft-libs pina também `5f80b9b`; os dois crates resolvem por path) |
| `libs/rustls-rustcrypto` | fork da suíte (provider TLS 100% Rust) | `0.0.2-alpha` |
| `assets/craft-fonts` | `storytold/craft-fonts` (build input `CRAFT_FONTS_DIR`) | `b076417` |

`craft-update` é **fork permanente consciente** e fica fora do sync: bugfix vai direto na fonte
(`local-libs`) e é replicado com `sync.ps1 -Mode push` (guarda de hash; a cópia aqui é espelho).
Nunca edite `libs/craft-libs/crates/update` à mão.

## Divergências do rebrand (o que é nosso)

- Identidade: crates `localphoto-*`, binários `localphoto`/`localphoto-cli`, app id
  `com.localphoto.app` (desktop entry, Ícone hicolor, Wayland app id, Info.plist, pacman),
  ProgIds/UpgradeCode do MSI, chaves de registro (`Software\Local Photo` +
  `Software\Anon5T4R\LocalPhoto\InstallDir`, App Paths), pasta de configuração `LocalPhoto`.
- CI próprio: sem `craft-common-actions`, sem sonarcloud, sem nix/flake, sem jobs macOS/FreeBSD/
  windows-arm64/web. Release por **tag** (o upstream usa a branch `release` + draft).
- `libs/` e `assets/craft-fonts/` vendorizados (o upstream busca os dois por git no CI).
- `cargo xtask fmt` (escopo = membros do workspace) no lugar de `cargo fmt --all`: os caminhos
  vendorizados mantêm o estilo de origem.
- Links de comunidade: os menus/About apontam só para este repositório (o upstream tinha site e
  Discord próprios).
- `packaging/windows/package.ps1` monta o zip portable com `ZipArchive` e barras `/` (o
  `Compress-Archive` do PS 5.1 grava `\`, que o updater recusa).
- `.github/`, README, AGENTS/CLAUDE e `docs/releasing.md` reescritos para a suíte.

## Pendências/gaps conhecidos desta porta

- Assinatura minisign dos assets: fail-open até os secrets `MINISIGN_KEY`/`MINISIGN_PASSPHRASE`
  existirem neste repo (ver `docs/releasing.md`).
- macOS: `packaging/macos/dmg/dmg-layout.DS_Store` ainda carrega o nome upstream no layout do
  Finder (regenerar exige macOS); o bundle em si já é `Local Photo.app`.
- Testes rodam no CI só no Linux (como no upstream); no Windows o job compila `cargo test
  --no-run` — os testes de UI renderizam por wgpu e o runner não tem GPU.
- `docs/images/` e `docs/showcase/` são capturas do app upstream (título antigo nos pixels);
  regenerar quando o visual estabilizar.
