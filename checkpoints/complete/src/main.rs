mod components;
mod add_channel;

use std::panic;
use add_channel::AddChannel;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{channel::Channel, channel_list::ChannelList, item::Item};

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
        <div class="content">
            <Suspense>
                <ChannelList />
            </Suspense>
            if let Route::Channel { url } = &route {
                <Suspense>
                    <Channel url={url.clone()} />
                </Suspense>
            }
            if let Route::Item { url, id } = route {
                <Suspense>
                    <Channel url={url.clone()} />
                    <Item url={url.clone()} {id} />
                </Suspense>
            }
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}