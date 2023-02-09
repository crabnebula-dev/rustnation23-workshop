use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::{channel::Channel, channels::Channels, item::Item, titlebar::TitleBar}, add_channel::AddChannel};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/channel/*url")]
    Channel { url: Url },
    #[at("/item/*url/:id")]
    Item { url: Url, id: usize },
    #[at("/add-channel")]
    AddChannel,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    if route == Route::AddChannel {
        return html! { <AddChannel /> };
    }

    html! {
        <>
            <TitleBar />
            <div class="content">
                <aside>
                <Suspense>
                    <Channels />
                </Suspense>
                </aside>
                <aside>
                    if let Route::Channel { url } | Route::Item { url, .. } = &route {
                        <Suspense>
                            <Channel url={url.clone()} />
                        </Suspense>
                    }
                </aside>
                <main>
                    if let Route::Item { url, id } = route {
                        <Suspense>
                            <Item url={url.clone()} {id}/>
                        </Suspense>
                    }
                </main>
            </div>
        </>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
