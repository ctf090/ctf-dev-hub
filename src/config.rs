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

/// Aceita só links http(s) e mailto:. Qualquer outra coisa (javascript:, data:,
/// vbscript:...) vira "#", assim um config.json adulterado não executa código no clique.
pub fn safe_url(raw: &str) -> String {
    let u = raw.trim();
    // Remove caracteres de controle/espaços que alguns navegadores ignoram dentro do esquema
    // (ex.: "java\tscript:") antes de comparar.
    let compact: String = u.chars().filter(|c| !c.is_control() && !c.is_whitespace()).collect();
    let l = compact.to_ascii_lowercase();
    if l.starts_with("https://") || l.starts_with("http://") || l.starts_with("mailto:") {
        compact
    } else {
        "#".to_string()
    }
}

/// Imagem local (/assets/...) ou https. Evita esquemas estranhos e "//outro-site".
pub fn safe_asset(raw: &str) -> String {
    let u = raw.trim();
    let local = u.starts_with('/') && !u.starts_with("//") && !u.contains("..");
    if local || u.to_ascii_lowercase().starts_with("https://") {
        u.to_string()
    } else {
        String::new()
    }
}

/// IDs de playlist do Spotify têm 22 caracteres alfanuméricos. Qualquer outra coisa
/// é descartada (a janela da playlist some), o que impede alterar o caminho do iframe.
pub fn safe_spotify_id(raw: &str) -> String {
    let id = raw.trim();
    if id.len() == 22 && id.chars().all(|c| c.is_ascii_alphanumeric()) {
        id.to_string()
    } else {
        String::new()
    }
}

impl Config {
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut cfg: Config =
            serde_json::from_str(text).map_err(|e| format!("config.json com erro: {e}"))?;
        cfg.sanitize();
        Ok(cfg)
    }

    /// Limpa tudo que vira link, imagem ou parte de URL antes de ir para a página.
    fn sanitize(&mut self) {
        self.avatar = safe_asset(&self.avatar);
        self.credit_url = safe_url(&self.credit_url);
        self.spotify_playlist_id = safe_spotify_id(&self.spotify_playlist_id);
        for s in &mut self.socials {
            s.url = safe_url(&s.url);
        }
        for l in &mut self.links {
            l.url = safe_url(&l.url);
        }
    }
}
