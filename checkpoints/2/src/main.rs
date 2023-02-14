mod components;

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

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    html! {
        <div class="content">
            <ChannelList />
            if let Route::Channel { .. } = &route {
                <Suspense>
                    <Channel />
                </Suspense>
            }
            if let Route::Item { .. } = route {
                <Suspense>
                    <Channel />
                    <Item />
                </Suspense>
            }
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    switch(Route::Item { url: Url::parse("https://tauri.app/blog/rss.xml").unwrap(), id: 0 })
    // html! {
    //     <BrowserRouter>
    //         <Switch<Route> render={switch} />
    //     </BrowserRouter>
    // }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
