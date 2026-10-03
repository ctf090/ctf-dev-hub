//! Funções de navegador via wasm-bindgen / web-sys / js-sys (zero JS escrito à mão).

use js_sys::{Function, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, MouseEvent, Response};

fn err(msg: &str) -> JsValue {
    JsValue::from_str(msg)
}

/// Copia texto para a área de transferência (navigator.clipboard.writeText).
/// Só funciona em HTTPS/localhost, o que o GitHub Pages já oferece.
pub async fn copy_text(text: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| err("sem window"))?;
    let navigator: JsValue = window.navigator().into();

    let clipboard = Reflect::get(&navigator, &JsValue::from_str("clipboard"))?;
    if clipboard.is_undefined() || clipboard.is_null() {
        return Err(err("clipboard indisponível"));
    }

    let write_text: Function = Reflect::get(&clipboard, &JsValue::from_str("writeText"))?
        .dyn_into()
        .map_err(|_| err("writeText indisponível"))?;

    let promise: Promise = write_text
        .call1(&clipboard, &JsValue::from_str(text))?
        .dyn_into()
        .map_err(|_| err("writeText não retornou Promise"))?;

    JsFuture::from(promise).await?;
    Ok(())
}

/// Baixa um arquivo de texto (usado para ler o config.json).
/// O `?t=` evita que o navegador mostre uma versão antiga depois de você editar.
pub async fn fetch_text(url: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or("sem window")?;
    let url = format!("{url}?t={}", js_sys::Date::now() as u64);

    let resp = JsFuture::from(window.fetch_with_str(&url))
        .await
        .map_err(|_| format!("não consegui abrir {url}"))?;
    let resp: Response = resp.dyn_into().map_err(|_| "resposta inválida")?;

    if !resp.ok() {
        return Err(format!("config.json não encontrado (HTTP {})", resp.status()));
    }

    let text = JsFuture::from(resp.text().map_err(|_| "sem texto na resposta")?)
        .await
        .map_err(|_| "falha ao ler o config.json")?;
    text.as_string().ok_or_else(|| "config.json vazio".to_string())
}

/// Troca o título da aba.
pub fn set_title(title: &str) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        doc.set_title(title);
    }
}

/// Guarda a posição do mouse dentro da janela em --mx/--my (o CSS usa isso
/// para desenhar o brilho que segue o cursor).
pub fn set_spot(el: &HtmlElement, ev: &MouseEvent) {
    let r = el.get_bounding_client_rect();
    let x = f64::from(ev.client_x()) - r.left();
    let y = f64::from(ev.client_y()) - r.top();
    let style = el.style();
    let _ = style.set_property("--mx", &format!("{x:.0}px"));
    let _ = style.set_property("--my", &format!("{y:.0}px"));
}
