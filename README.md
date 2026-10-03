<div align="center">

# ctf-dev-hub

**Meu hub pessoal de desenvolvedor.**
Projetos, links, redes sociais e um pouco sobre mim, tudo em um só lugar.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Leptos](https://img.shields.io/badge/Leptos-EF3939?style=for-the-badge)
![Status](https://img.shields.io/badge/status-em%20desenvolvimento-orange?style=for-the-badge)

</div>

---

## Sobre

O **ctf-dev-hub** é o meu site pessoal, funcionando como cartão de visitas e vitrine de projetos. Ele reúne:

- Quem eu sou e no que trabalho
- Meus projetos
- Links e redes sociais

O site é escrito inteiramente em **Rust**, compilado para **WebAssembly** e renderizado no navegador (CSR). Isso gera um bundle pequeno e rápido de carregar, sem depender de JavaScript escrito à mão.

## Tecnologias

| Camada | Tecnologia |
| --- | --- |
| Linguagem | Rust (edition 2021) |
| Framework de UI | [Leptos](https://leptos.dev/) 0.8 (modo CSR) |
| Build / servidor de dev | [Trunk](https://trunkrs.dev/) |
| Alvo de compilação | `wasm32-unknown-unknown` |
| Interop com o navegador | `wasm-bindgen`, `web-sys`, `js-sys` |
| Dados | `serde` + `serde_json` |
| Estilo | CSS puro (`style.css`) |
| CI/CD | GitHub Actions (`.github/workflows`) |

O perfil de release é otimizado para tamanho (`opt-level = "z"`, LTO, `codegen-units = 1` e `panic = "abort"`), deixando o `.wasm` o menor possível.

## Estrutura do projeto

```
ctf-dev-hub/
├── .github/workflows/   # Automações (CI/CD)
├── assets/              # Imagens e arquivos estáticos
├── scripts/             # Scripts auxiliares
├── src/                 # Código-fonte em Rust (Leptos)
├── index.html           # Ponto de entrada processado pelo Trunk
├── style.css            # Estilos do site
├── Cargo.toml           # Dependências e perfil de build
├── Trunk.toml           # Configuração do Trunk
└── README.md
```

## Como rodar localmente

### Pré-requisitos

- [Rust](https://rustup.rs/) (toolchain estável)
- Target WebAssembly
- Trunk

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Passo a passo

```bash
# 1. Clone o repositório
git clone https://github.com/ctf090/ctf-dev-hub.git
cd ctf-dev-hub

# 2. Inicie o servidor de desenvolvimento com hot reload
trunk serve --open
```

O site abre em `http://127.0.0.1:8080`.

## Build de produção

```bash
trunk build --release
```

Os arquivos finais são gerados na pasta `dist/`, prontos para publicar em qualquer hospedagem estática (GitHub Pages, Netlify, Vercel, Cloudflare Pages etc.).

## Personalização

- **Conteúdo e componentes:** edite os arquivos em `src/`
- **Visual:** ajuste o `style.css`
- **Imagens:** coloque os arquivos em `assets/`
- **Caminho público do site:** altere `public_url` no `Trunk.toml` (por exemplo, `"/ctf-dev-hub/"` se for hospedar em subpasta no GitHub Pages)

## Contato

- GitHub: [@ctf090](https://github.com/ctf090)

<!-- Adicione aqui seus outros links: Instagram, TikTok, Discord, e-mail etc. -->

---

<div align="center">

Feito com Rust 🦀

</div>
