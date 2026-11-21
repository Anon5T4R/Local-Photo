# ROADMAP — Local Photo

Fork do lightcraft, rebrandado para a suíte Local. Rust puro + egui, só desktop: Windows (MSI e portable) e Linux
(AppImage, deb, rpm, pacman, tar.gz). Biblioteca, revelação de RAW, máscaras, merge HDR/panorama, impressão e export,
com catálogo próprio e importação do catálogo do Lightroom. Como é construído: `docs/architecture.md`. O que vem do
upstream: `docs/UPSTREAM-SYNC.md`.

**Estágio: alpha.** Os fluxos centrais funcionam de ponta a ponta: importar, escolher, revelar, salvar e reabrir,
aplicar looks em lote e exportar. O que está abaixo é o caminho até "pronto para trabalho de verdade".

## O que falta (por área, sem ordem de dificuldade)

- **RAW.** A cobertura por modelo está medida em `docs/raw-parity.md` (com o corpus pinado em `docs/raw/`). ORF
  comprimido e NEF HE abrem só como prévia. Variantes raras de CR3 também. Muitos modelos decodificam, mas sem
  verificação num arquivo real.
- **Cor da câmera.** Falta calibração medida para os modelos populares: a maioria usa matriz estimada, e só alguns
  corpos têm perfil próprio. A renderização foi ajustada a olho, sem uma bateria de fidelidade contra uma referência.
- **Lentes.** Sem base própria de perfis de lente. Hoje a correção vem dos dados que a câmera grava (Sony, Fujifilm,
  Nikon: docs `*-lens-corrections.md`).
- **Máscaras com IA.** Assunto, céu e pessoas são heurísticas. O modelo e a forma de distribuí-lo seguem o padrão de IA
  local da suíte ("Instalar de arquivo…", como o AI Denoise).
- **Remover, curar e Enhance.** Mais enxutos que os do Lightroom.
- **Módulos de saída.** Mapa, livro, publicação e tethering não existem. Slideshow e impressão existem.
- **Vídeo.** Não importa nem edita vídeo.
- **Localização.** Inglês completo. Os outros catálogos (`crates/ui-egui/locales`) estão parciais. Hindi, árabe e
  coreano não têm a forma das letras resolvida (shaping/RTL, face Hangul).

## Referências

`docs/architecture.md` · `docs/raw-parity.md` · `docs/control-protocol.md` · `docs/mcp.md` · `docs/denoise.md` ·
`docs/lightroom-catalog-import.md` · `docs/localization.md` · `docs/UPSTREAM-SYNC.md`
