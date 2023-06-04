pub mod app;
pub mod components;
pub mod models;
pub mod pages;
pub mod server_functions;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::app::*;
        use leptos::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            log!("hydrate mode - hydrating");

            leptos::mount_to_body(|cx| {
                view! { cx,  <App/> }
            });
        }
    }
}
