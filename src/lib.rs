pub mod app;
pub mod chat;
pub mod error_template;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate()
{
	use crate::app::*;
	console_error_panic_hook::set_once();
	let level = log::Level::Trace;
	console_log::init_with_level(level).expect("Failed to initialize logger");
	log::debug!("Logging online: ≥{}", level);
	leptos::mount::hydrate_body(App);
}
