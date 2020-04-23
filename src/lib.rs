
pub mod app;
pub mod types;
pub mod error;
pub mod services;
pub mod components;

use wasm_bindgen::prelude::*;
use web_logger;

pub use app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<App>();
    Ok(())
}
