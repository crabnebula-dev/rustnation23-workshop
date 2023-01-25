use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { "Hello World" },
        Route::NotFound => html! { "Not Found" },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::Renderer::<App>::new().render();
}
