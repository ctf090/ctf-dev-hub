mod app;
mod components;
mod config;
mod util;

fn main() {
    // Mostra panics no console só em desenvolvimento (no build release não expõe detalhes).
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
