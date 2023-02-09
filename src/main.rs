mod app;
mod components;
mod add_channel;

extern crate console_error_panic_hook;

use app::App;
use std::panic;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
