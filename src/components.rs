//! Um componente Leptos para cada bloco da página.

use leptos::prelude::*;
use leptos::task::spawn_local;
use std::time::Duration;

use crate::config::{LinkItem, Social};
use crate::util;

// ------------------------------------------------------------
// Toast compartilhado (o App cria e fornece via contexto)
// ------------------------------------------------------------
#[derive(Clone, Copy)]
pub struct Toaster {
    message: RwSignal<String>,
    visible: RwSignal<bool>,
    ticket: RwSignal<u32>, // evita que um toast antigo esconda um novo
}

impl Toaster {
    pub fn new() -> Self {
        Self {
            message: RwSignal::new(String::new()),
            visible: RwSignal::new(false),
            ticket: RwSignal::new(0),
        }
    }

    pub fn show(&self, text: &str) {
        let id = self.ticket.get_untracked().wrapping_add(1);
        self.ticket.set(id);
        self.message.set(text.to_string());
        self.visible.set(true);

        let (visible, ticket) = (self.visible, self.ticket);
        set_timeout(
            move || {
                if ticket.get_untracked() == id {
                    visible.set(false);
                }
            },
            Duration::from_millis(2200),
        );
    }
}

#[component]
pub fn Toast() -> impl IntoView {
    let toaster = expect_context::<Toaster>();
    view! {
        <div
            class="toast"
            class:show=move || toaster.visible.get()
            role="status"
            aria-live="polite"
        >
            {move || toaster.message.get()}
        </div>
    }
}

// ------------------------------------------------------------
// Janela estilo macOS — usada em TODOS os blocos da página.
//   bolinha vermelha: balança e avisa que não fecha
//   bolinha amarela:  minimiza / expande a janela
//   bolinha verde:    dá um "zoom" (pulso)
//   o brilho laranja segue o mouse dentro da janela
// ------------------------------------------------------------
#[component]
pub fn Window(
    #[prop(into)] title: String,
    /// posição na animação de entrada (0, 1, 2…): quanto maior, mais tarde entra
    #[prop(default = 0)]
    i: usize,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let toaster = expect_context::<Toaster>();
    let minimized = RwSignal::new(false);
    let shaking = RwSignal::new(false);
    let zooming = RwSignal::new(false);
    let node = NodeRef::<leptos::html::Div>::new();

    let on_close = move |_| {
        shaking.set(true);
        toaster.show("Essa janela não fecha 😉");
        set_timeout(move || shaking.set(false), Duration::from_millis(500));
    };
    let on_minimize = move |_| minimized.update(|m| *m = !*m);
    let on_zoom = move |_| {
        zooming.set(true);
        set_timeout(move || zooming.set(false), Duration::from_millis(500));
    };

    view! {
        <section class=format!("win-wrap reveal {class}") style=format!("--i: {i}")>
            <div
                class="win"
                class:min=move || minimized.get()
                class:shake=move || shaking.get()
                class:zoom=move || zooming.get()
                node_ref=node
                on:mousemove=move |ev| {
                    if let Some(el) = node.get_untracked() {
                        util::set_spot(&el, &ev);
                    }
                }
            >
                <header class="win-bar">
                    <div class="lights">
                        <button type="button" class="light red" data-sym="×"
                                aria-label="Fechar" on:click=on_close></button>
                        <button type="button" class="light yellow" data-sym="−"
                                aria-label="Minimizar" on:click=on_minimize></button>
                        <button type="button" class="light green" data-sym="+"
                                aria-label="Zoom" on:click=on_zoom></button>
                    </div>
                    <h2 class="win-title">{title}</h2>
                    <span class="win-spacer" aria-hidden="true"></span>
                </header>
                <div class="win-body">
                    <div class="win-inner">{children()}</div>
                </div>
            </div>
        </section>
    }
}

// ------------------------------------------------------------
// Ícones sociais (qualquer nome desconhecido vira um ícone de link)
// ------------------------------------------------------------
#[component]
fn Icon(kind: String) -> impl IntoView {
    match kind.to_lowercase().as_str() {
        "instagram" => view! {
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                 stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <rect x="3" y="3" width="18" height="18" rx="5" />
                <circle cx="12" cy="12" r="4" />
                <circle cx="17.5" cy="6.5" r="1" fill="currentColor" stroke="none" />
            </svg>
        }
        .into_any(),
        "tiktok" => view! {
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M12.525.02c1.31-.02 2.61-.01 3.91-.02.08 1.53.63 3.09 1.75 4.17 1.12 1.11 2.7 1.62 4.24 1.79v4.03c-1.44-.05-2.89-.35-4.2-.97-.57-.26-1.1-.59-1.62-.93-.01 2.92.01 5.84-.02 8.75-.08 1.4-.54 2.79-1.35 3.94-1.31 1.92-3.58 3.17-5.91 3.21-1.43.08-2.86-.31-4.08-1.03-2.02-1.19-3.44-3.37-3.65-5.71-.02-.5-.03-1-.01-1.49.18-1.9 1.12-3.72 2.58-4.96 1.66-1.44 3.98-2.13 6.15-1.72.02 1.48-.04 2.96-.04 4.44-.99-.32-2.15-.23-3.02.37-.63.41-1.11 1.04-1.36 1.75-.21.51-.15 1.07-.14 1.61.24 1.64 1.82 3.02 3.5 2.87 1.12-.01 2.19-.66 2.77-1.61.19-.33.4-.67.41-1.06.1-1.79.06-3.57.07-5.36.01-4.03-.01-8.05.02-12.07z" />
            </svg>
        }
        .into_any(),
        "youtube" => view! {
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z" />
            </svg>
        }
        .into_any(),
        "github" => view! {
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" />
            </svg>
        }
        .into_any(),
        "x" | "twitter" => view! {
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M18.901 1.153h3.68l-8.04 9.19L24 22.846h-7.406l-5.8-7.584-6.638 7.584H.474l8.6-9.83L0 1.154h7.594l5.243 6.932ZM17.61 20.644h2.039L6.486 3.24H4.298Z" />
            </svg>
        }
        .into_any(),
        "email" | "mail" => view! {
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                 stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <rect x="3" y="5" width="18" height="14" rx="3" />
                <path d="M4 7l8 6 8-6" />
            </svg>
        }
        .into_any(),
        _ => view! {
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                 stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <path d="M10 14a4 4 0 0 0 5.66 0l3-3a4 4 0 0 0-5.66-5.66l-1 1" />
                <path d="M14 10a4 4 0 0 0-5.66 0l-3 3a4 4 0 0 0 5.66 5.66l1-1" />
            </svg>
        }
        .into_any(),
    }
}

// ------------------------------------------------------------
// Barra lateral: tudo sobre você (logo, nick, tag, bio, chips, redes)
// ------------------------------------------------------------
#[component]
pub fn Profile(
    name: String,
    tag: String,
    bio: String,
    avatar: String,
    techs: Vec<String>,
    socials: Vec<Social>,
) -> impl IntoView {
    // O efeito de digitação precisa saber quantos caracteres a tag tem.
    let tag_len = tag.chars().count().max(1);

    view! {
        <Window title="sobre mim" i=0>
            <div class="profile">
                {(!avatar.is_empty()).then(|| view! {
                    <div class="avatar">
                        <img src=avatar alt=name.clone() width="128" height="128" />
                    </div>
                })}
                <h1 class="name">{name}</h1>
                {(!tag.is_empty()).then(|| view! {
                    <p class="tag" style=format!("--n: {tag_len}")>{tag}</p>
                })}
                {(!bio.is_empty()).then(|| view! { <p class="bio">{bio}</p> })}

                {(!techs.is_empty()).then(|| view! {
                    <ul class="chips">
                        {techs
                            .into_iter()
                            .enumerate()
                            .map(|(n, t)| view! {
                                <li class="chip" style=format!("--i: {n}")>{t}</li>
                            })
                            .collect_view()}
                    </ul>
                })}

                {(!socials.is_empty()).then(|| view! {
                    <nav class="socials" aria-label="Redes sociais">
                        {socials
                            .into_iter()
                            .enumerate()
                            .map(|(n, s)| view! {
                                <a class="social" style=format!("--i: {n}") href=s.url
                                   target="_blank" rel="noopener noreferrer"
                                   aria-label=s.label.clone() title=s.label>
                                    <Icon kind=s.icon />
                                </a>
                            })
                            .collect_view()}
                    </nav>
                })}
            </div>
        </Window>
    }
}

// ------------------------------------------------------------
// Seção LINKS (a numeração é automática)
// ------------------------------------------------------------
#[component]
pub fn LinksSection(links: Vec<LinkItem>, i: usize) -> impl IntoView {
    view! {
        <Window title="links" i=i>
            <div class="links-list">
                {links
                    .into_iter()
                    .enumerate()
                    .map(|(n, l)| {
                        view! {
                            <a class="link-row" style=format!("--i: {n}")
                               href=l.url target="_blank" rel="noopener noreferrer">
                                <span class="link-num">{format!("{:02}", n + 1)}</span>
                                <span class="link-body">
                                    <strong class="link-title">{l.title}</strong>
                                    <span class="link-desc">{l.desc}</span>
                                </span>
                                <svg class="link-arrow" viewBox="0 0 24 24" fill="none"
                                     stroke="currentColor" stroke-width="2"
                                     stroke-linecap="round" stroke-linejoin="round"
                                     aria-hidden="true">
                                    <path d="M5 12h14M13 6l6 6-6 6" />
                                </svg>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </Window>
    }
}

// ------------------------------------------------------------
// Terminal com comando copiável
// ------------------------------------------------------------
#[component]
pub fn CopyCard(command: String, i: usize) -> impl IntoView {
    let toaster = expect_context::<Toaster>();
    let copied = RwSignal::new(false);
    let to_copy = command.clone();

    let on_copy = move |_| {
        let cmd = to_copy.clone();
        spawn_local(async move {
            match util::copy_text(&cmd).await {
                Ok(()) => {
                    copied.set(true);
                    toaster.show("Comando copiado!");
                    set_timeout(move || copied.set(false), Duration::from_millis(1800));
                }
                Err(_) => toaster.show("Não foi possível copiar"),
            }
        });
    };

    view! {
        <Window title="terminal" i=i>
            <div class="terminal-body">
                <span class="prompt">"$"</span>
                <code>{command}</code>
                <button type="button" class="copy-btn"
                        class:copied=move || copied.get()
                        on:click=on_copy>
                    {move || if copied.get() { "Copiado!" } else { "copiar" }}
                </button>
            </div>
        </Window>
    }
}

// ------------------------------------------------------------
// Playlist do Spotify
// ------------------------------------------------------------
#[component]
pub fn SpotifyEmbed(playlist_id: String, i: usize) -> impl IntoView {
    let src = format!(
        "https://open.spotify.com/embed/playlist/{playlist_id}?utm_source=generator&theme=0"
    );
    view! {
        <Window title="playlist" i=i class="spotify-win">
            <iframe
                class="spotify-frame"
                src=src
                title="Playlist no Spotify"
                height="352"
                allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
            ></iframe>
        </Window>
    }
}

// ------------------------------------------------------------
// Rodapé
// ------------------------------------------------------------
#[component]
pub fn Footer(name: String, credit_text: String, credit_url: String) -> impl IntoView {
    let year = js_sys::Date::new_0().get_full_year();
    view! {
        <footer class="footer">
            <p>{format!("© {year} {name}")}</p>
            {(!credit_text.is_empty()).then(|| view! {
                <p>
                    <a href=credit_url target="_blank" rel="noopener noreferrer">{credit_text}</a>
                </p>
            })}
        </footer>
    }
}
