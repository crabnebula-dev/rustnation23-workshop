use serde::Serialize;
use url::Url;
use yew::{prelude::*, suspense::use_future_with_deps};
use tauri_sys::tauri;

use crate::components::item_preview::ItemPreview;

#[derive(Serialize)]
struct FetchChannelArgs<'a> {
    url: &'a Url,
}

#[derive(Debug, Properties, PartialEq)]
pub struct ChannelProps {
    pub url: Url,
}

#[function_component]
pub fn Channel(props: &ChannelProps) -> HtmlResult {
    let result = use_future_with_deps(
        |url| async move {
            tauri::invoke::<_, rss::Channel>("fetch_channel", &FetchChannelArgs { url: &url }).await
        },
        props.url.clone(),
    )?;

    let Ok(channel) = &*result else {
        log::error!("Failed to fetch channel {:?}", result.as_ref().unwrap_err());
        return Ok(html! { <>{"failed to fetch channel"}{result.as_ref().unwrap_err().to_string()}</> });
    };

    let items = channel.items.iter().enumerate().map(|(id, item)| {
        html! {
            <ItemPreview
                {id}
                channel_url={props.url.clone()}
                channel_title={channel.title.clone()}
                title={item.title.clone()}
                pub_date={item.pub_date.clone()}
                description={item.description.clone()}
            />
        }
    });

    Ok(html! {
        <aside class="channel">
            <strong class="channel-title">{&channel.title}</strong>
            <ul>
                {items.collect::<Html>()}
            </ul>
        </aside>
    })
}
