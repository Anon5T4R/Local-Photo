# Releasing Local Photo

O pipeline de release é [`.github/workflows/release.yml`](../.github/workflows/release.yml).
Ele roda em todo push de tag `v*` e publica a GitHub Release da tag.

## Prepare e dispare um release

1. Merge das mudanças prontas em `main`.
2. Suba a versão do workspace no `Cargo.toml` (`cargo xtask version set X.Y.Z`) e regenere o
   `Cargo.lock` (`cargo update --workspace --offline`; nunca apagar o lock).
3. Commit + push em `main`.
4. `git tag vX.Y.Z && git push origin vX.Y.Z` (uma tag por push: mais de 3 tags juntas não geram
   evento de push e o Release não dispara).
5. Confira os jobs (`windows`, `linux`, `arch`) e o job `publish`. A release é criada com
   `--generate-notes` e os assets são enviados com `--clobber` (idempotente: rodar de novo na
   mesma tag re-sobe os arquivos).

**Bump antes da tag, sempre**: a tag aponta para o HEAD, e os assets levam o nome da versão do
`Cargo.toml` (`localphoto-<ver>-*`). Taggear sem bump publica assets com o número antigo.

Tag não-publicada pode ser movida sem cerimônia (delete local + `git push origin :refs/tags/vX` +
re-tag no commit bom + push da tag sozinha); só release **já publicada** exige apagar a release antes.

## O que o workflow builda

Só Windows, Linux e Arch (os mesmos jobs dos irmãos da suíte). Os scripts de `packaging/` para
macOS, FreeBSD, Flatpak e web continuam no repo para uso manual, mas não são exercitados no CI
deste fork.

- **Windows** (`packaging/windows/package.ps1`): MSI per-machine (WiX 5.0.2 + extensões UI/Util,
  UpgradeCode próprio) e o zip portable. Os dois vão em `dist/release/`.
- **Linux** (`packaging/linux/package.sh --formats "appimage deb rpm tar"`, Ubuntu 22.04, glibc
  2.35+): `.AppImage` (+ `.zsync` quando há `zsyncmake`), `.deb` e `.rpm` (nfpm 2.48), e o
  `.tar.gz` com a árvore FHS.
- **Arch**: o job baixa o `.tar.gz` do job `linux`, carimba `pkgver`/`source`/`sha256sums` a partir
  do `Cargo.toml` (padrão da suíte: PKGBUILD nunca fica com versão velha) e roda `makepkg` como
  usuário `builder`; o asset sai como `localphoto-<ver>-linux-x86_64.pkg.tar.zst`.
- **publish**: `SHA256SUMS.txt` + assinatura minisign de todo asset com os secrets
  `MINISIGN_KEY`/`MINISIGN_PASSPHRASE`, auto-verificada contra `packaging/minisign.pub`. **Política
  transitória**: sem secrets (ou chave que não bate) a release sai SEM assinatura, com warning no
  log — nunca com assinatura inválida.

## Fontes (CJK)

O craft-fonts está vendored em `assets/craft-fonts` (rev `b076417`: BIZ UDPGothic, BIZ UDMincho,
Shippori Mincho, Noto Sans CJK SC, NanumGothic, Noto Sans Indic/Arabic/Symbols). Os jobs passam
`CRAFT_FONTS_DIR=$GITHUB_WORKSPACE/assets/craft-fonts`; sem ele o build compila mas chinês/japonês
saem como tofu. Ao atualizar o craft-fonts: trocar a árvore inteira, bump em
`assets/ATTRIBUTION.md` e rodar `CRAFT_FONTS_DIR=assets/craft-fonts cargo test -p localphoto-ui-egui i18n`
(o teste de cobertura de glifos). Os scripts de `packaging/` também aceitam `../craft-fonts` e um
`CRAFT_FONTS_DIR` externo.

## Assinatura de código

Não há assinatura Authenticode/notarização neste fork (decisão da suíte: builds unsigned, o
checksum do GitHub é a âncora de integridade). Se um dia houver: `packaging/windows/sign.ps1` e
`packaging/macos/import-cert.sh` leem as variáveis de certificado do ambiente
(`WINDOWS_CERTIFICATE`/`WINDOWS_CERTIFICATE_PASSWORD`, `AZURE_*`, `APPLE_*`) e degradam para
artefato sem assinatura quando elas faltam.

**Bootstrap do auto-update:** uma instalação feita com um release anterior à política unsigned do
`craft-update` precisa instalar, uma única vez, um release que já a contenha; daí em diante o
auto-update flui.
