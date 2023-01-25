use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

impl Route {
    pub const PRERENDER: &[Self] = &[Self::Home, Self::NotFound];
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { "Hello World" },
        Route::NotFound => html! { "Not Found" },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <Switch<Route> render={switch} />
        </main>
    }
}