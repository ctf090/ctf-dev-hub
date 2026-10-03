//! Estrutura do arquivo assets/config.json.
//! Você NÃO precisa mexer aqui: edite só o config.json e recarregue a página.
//! Campos que faltarem ficam vazios, e seções vazias somem da página.

use serde::Deserialize;

/// Caminho do arquivo de configuração (dentro de assets/).
pub const CONFIG_URL: &str = "/assets/config.json";

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub name: String,
    pub tag: String,
    pub bio: String,
    pub avatar: String,
    pub techs: Vec<String>,
    pub socials: Vec<Social>,
    pub links: Vec<LinkItem>,
    pub command: String,
    pub spotify_playlist_id: String,
    pub credit_text: String,
    pub credit_url: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Social {
    pub label: String,
    pub url: String,
    /// instagram, tiktok, youtube, github, x, email — qualquer outro vira um ícone de link.
    pub icon: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct LinkItem {
    pub title: String,
    pub desc: String,
    pub url: String,
}

impl Config {
    pub fn parse(text: &str) -> Result<Self, String> {
        serde_json::from_str(text).map_err(|e| format!("config.json com erro: {e}"))
    }
}
