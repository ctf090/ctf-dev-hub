# bio-site

Tudo que você edita fica em **`assets/config.json`**: nick, tag, bio, tecnologias, redes sociais, links, comando do terminal e playlist.
Salvou o arquivo, recarregou a página, mudou. Não precisa recompilar o Rust.

- **Adicionar rede social:** copie uma linha de `socials` e troque `label`, `icon` e `url`. Ícones prontos: `instagram`, `tiktok`, `youtube`, `github`, `x`, `email`. Qualquer outro nome mostra um ícone de link.
- **Remover rede social ou link:** apague a linha inteira (cuidado com a vírgula: a última linha da lista não leva vírgula).
- **Esconder uma janela:** deixe vazio, por exemplo `"command": ""` esconde o terminal, `"spotify_playlist_id": ""` esconde a playlist e `"links": []` esconde os links.
- **Playlist:** o ID é o pedaço do link depois de `/playlist/` e antes do `?`.
- **Trocar a foto ou o fundo:** substitua `assets/logo.jpg` e `assets/bg.jpg`.

## Rodar / publicar

    rustup target add wasm32-unknown-unknown
    cargo install trunk
    trunk serve            # desenvolvimento
    trunk build --release  # gera a pasta dist/ (o GitHub Actions faz isso sozinho a cada push)

## Bolinhas das janelas

Vermelha balança a janela, amarela minimiza e abre, verde dá um pulinho.
