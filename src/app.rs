//! Carrega o assets/config.json e monta a página:
//! barra lateral (você) + coluna de janelas (links, terminal, playlist).

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::*;
use crate::config::{Config, CONFIG_URL};
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    // Toast global, acessível por qualquer componente via expect_context.
    provide_context(Toaster::new());

    // None = carregando · Some(Err) = config com problema · Some(Ok) = pronto
    let state: RwSignal<Option<Result<Config, String>>> = RwSignal::new(None);

    spawn_local(async move {
        let result = match util::fetch_text(CONFIG_URL).await {
            Ok(text) => Config::parse(&text),
            Err(e) => Err(e),
        };
        if let Ok(cfg) = &result {
            util::set_title(&format!("{} | Links", cfg.name));
        }
        state.set(Some(result));
    });

    view! {
        // Fundo animado: duas luzes coloridas à deriva.
        <div class="orbs" aria-hidden="true">
            <span class="orb orb-a"></span>
            <span class="orb orb-b"></span>
        </div>

        {move || match state.get() {
            None => view! { <div class="loading"><span class="spinner"></span></div> }.into_any(),
            Some(Err(msg)) => view! { <ErrorBox msg=msg /> }.into_any(),
            Some(Ok(cfg)) => view! { <Page cfg=cfg /> }.into_any(),
        }}

        <Toast />
    }
}

#[component]
fn ErrorBox(msg: String) -> impl IntoView {
    view! {
        <div class="layout single">
            <Window title="erro" i=0>
                <p class="error-text">{msg}</p>
            </Window>
        </div>
    }
}

#[component]
fn Page(cfg: Config) -> impl IntoView {
    let Config {
        name, tag, bio, avatar, techs, socials, links,
        command, spotify_playlist_id, credit_text, credit_url,
    } = cfg;

    let footer_name = name.clone();

    // Ordem de entrada: o perfil é 0; as janelas da direita vêm em seguida.
    let has_links = !links.is_empty();
    let has_cmd = !command.trim().is_empty();
    let has_spotify = !spotify_playlist_id.trim().is_empty();
    let links_i = 1;
    let cmd_i = links_i + has_links as usize;
    let spotify_i = cmd_i + has_cmd as usize;

    view! {
        <div class="layout">
            <aside class="sidebar">
                <Profile name=name tag=tag bio=bio avatar=avatar techs=techs socials=socials />
            </aside>

            <div class="content">
                {has_links.then(|| view! { <LinksSection links=links i=links_i /> })}
                {has_cmd.then(|| view! { <CopyCard command=command i=cmd_i /> })}
                {has_spotify.then(|| view! {
                    <SpotifyEmbed playlist_id=spotify_playlist_id i=spotify_i />
                })}
            </div>
        </div>

        <Footer name=footer_name credit_text=credit_text credit_url=credit_url />
    }
}
