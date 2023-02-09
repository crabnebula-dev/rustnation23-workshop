use serde::Serialize;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[derive(Debug, Properties, PartialEq)]
pub struct ChannelPreviewProps {
    pub url: Url,
}

#[derive(Serialize)]
pub struct FetchChannelIconsArgs<'a> {
    pub url: &'a Url,
}

#[function_component]
pub fn ChannelPreview(props: &ChannelPreviewProps) -> HtmlResult {
    let icon_url = format!("https://icons.duckduckgo.com/ip3/{}.ico", props.url.host_str().unwrap());

    Ok(html! {
        <li key={props.url.as_str()} class="channel-preview">
            <Link<Route> to={Route::Channel { url: props.url.clone() }}>
                <img width="32" height="32" class="favicon" src={icon_url.to_string()}/>
            </Link<Route>>
        </li>
    })
}
