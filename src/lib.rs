use console_error_panic_hook::set_once;
use leptos::mount_to_body;
use wasm_bindgen::prelude::*;

pub mod app;
pub mod components;
mod analytics;

pub use app::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    set_once();
    mount_to_body(App);
}