use yew_router::BrowserRouter;
use yew::prelude::*;
use yew::Renderer;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <ui::App />
        </BrowserRouter>
    }
}

fn main() {
    let renderer = Renderer::<App>::new();

    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    // hydrates everything under body element, removes trailing
    // elements (if any).
    renderer.hydrate();
}