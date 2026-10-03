mod app;
mod components;
mod config;
mod util;

fn main() {
    // Mostra panics do Rust no console do navegador (útil em desenvolvimento).
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
