# Sincronização com o upstream

Porta/rebrand da suíte Local: o `main` é nosso, o upstream é fonte de mudanças candidatas e nada
entra sem revisão. Regras gerais em `docs/planos/licoes-de-porta.md` (dev-notes).

## Base

- Upstream: `storytold/lightcraft`, commit **bb08a69** (`0.14.0`), sem tag no commit — a árvore foi
  copiada do `git archive` desse commit (sem histórico).
- Fork: `Styx-Studios/Local-Photo`, versão própria `0.1.x` (numeração nova evita auto-update falso
  para o upstream).
- Nada é lido de `C:\Dev\storytold` em build, teste ou release; a pasta é só referência local para
  a skill `sync-upstream`.

## Vendorizado (fora do sync normal)

| Caminho | Origem | Rev |
|---|---|---|
| `libs/craft-libs` | `storytold/craft-libs` | `1e6f1278` |
| `libs/craft-libs/crates/update` | **`Styx-Studios/local-libs`** (fonte única da suíte) | ver `HASHES.update` do repo |
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
  windows-arm64. Release por **tag** (o upstream usa a branch `release` + draft).
- `libs/` e `assets/craft-fonts/` vendorizados (o upstream busca os dois por git no CI).
- `cargo xtask fmt` (escopo = membros do workspace) no lugar de `cargo fmt --all`: os caminhos
  vendorizados mantêm o estilo de origem.
- Links de comunidade: os menus/About apontam só para este repositório (o upstream tinha site e
  Discord próprios).
- Ícone próprio (v0.1.2): iris de câmera branca em tile fúcsia, SVG desenhado neste repo (o upstream
  usava um lince gravado, arte herdada que saiu inteira — hicolor, `.ico`, `.icns`,
  `localphoto-1024.png`, DMG background). Geração: `packaging/icons.sh` + `cargo xtask ico` + packer
  de `.icns` por chunks PNG (sem macOS).
- MSI (v0.1.2): atalho na **área de trabalho** além do menu Iniciar (componente próprio, atalho
  não-advertised, key path no registro `Software\Local Photo\Shortcuts`), padrão do local-paint.
- `packaging/windows/package.ps1` monta o zip portable com `ZipArchive` e barras `/` (o
  `Compress-Archive` do PS 5.1 grava `\`, que o updater recusa).
- `crates/ui-egui/src/render.rs` (`request_needed`): `clippy::needless_bool` novo do stable 1.100
  reprovava o `if … { return false; } true` do upstream. Condição negada (sugestão do próprio
  lint), sem mudança de comportamento — divergência local mínima até o upstream acompanhar.
- `.github/`, README, AGENTS/CLAUDE e `docs/releasing.md` reescritos para a suíte.

## Registro de sync

A base continua `bb08a69`: só pontos escolhidos entram (skill `sync-upstream`), não um bump.

| Data | Alvo (lightcraft) | Pegos | Descartados relevantes |
|---|---|---|---|
| 2026-11-21 | `origin/main` do dia | série #2589 `de9b0a89` `7c74ae0a` `a31a4367` `5b8b9b52` → `8682269` (duplicata por conteúdo no catálogo do Lightroom); `8ffe73cc` → `89fbbc7` (lente X-S20, sensores X-S10/X-S20/X-M5) | `2d905e0a` `89d11ab1` (backup ZIP do .lrcat: não temos), `5c598189` `d7372407` `e689453e` `19985a3f` `690f6cfb` (log do catálogo: não temos), `6c3083eb` `5a242898` (dependem do `craft-raw`), `cb91d69e` (hot-pixel: não temos), `a57bd69d` (perf menor, depende do job novo) |

## Pendências/gaps conhecidos desta porta

- Assinatura minisign dos assets: fail-open até os secrets `MINISIGN_KEY`/`MINISIGN_PASSPHRASE`
  existirem neste repo (ver `docs/releasing.md`).
- macOS: `packaging/macos/dmg/dmg-layout.DS_Store` ainda carrega o nome upstream no layout do
  Finder (regenerar exige macOS); o bundle em si já é `Local Photo.app`.
- Testes rodam no CI só no Linux (como no upstream); no Windows o job compila `cargo test
  --no-run` — os testes de UI renderizam por wgpu e o runner não tem GPU. Três testes do upstream
  não passavam no Windows e foram corrigidos/limitados aqui: dois comparavam caminhos montados com
  `join("a/b")` (separador misto) e um espera detecção de hard link por nome diferente, que no
  Windows dependeria do file index (metadata instável) — a asserção de *admissão* ficou `#[cfg(unix)]`
  e a de *write-time* (caminho que apareceu) continua valendo nos dois.
- `docs/images/` e `docs/showcase/` são capturas do app upstream (título antigo nos pixels);
  regenerar quando o visual estabilizar.
- **Flake de carga (não suprimido):** `tests_segment::background_requests_never_wait_and_failures_come_back_as_messages`
  mede o tempo de enfileirar pedidos ao modelo (< 1 s) e estourou uma vez (1,8 s) no Windows
  local com a suíte inteira em paralelo; passa isolado e no binário do crate (1301 testes). Código
  intocado aqui — se flecar no CI, investigar contenção de I/O do modelo antes de mexer no limite.
- Testes locais pedem `CRAFT_FONTS_DIR=assets/craft-fonts` (o teste de acessibilidade desenha "▸",
  que só existe no set do craft-fonts; o CI já define a variável).
