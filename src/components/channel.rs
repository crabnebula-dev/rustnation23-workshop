use serde::Serialize;
use tauri_sys::tauri;
use url::Url;
use yew::{prelude::*, suspense::use_future_with_deps};

use crate::components::item_preview::ItemPreview;

#[derive(Debug, Properties, PartialEq)]
pub struct ChannelProps {
    pub url: Url,
}

#[derive(Serialize)]
struct FetchChannelArgs<'a> {
    url: &'a Url,
}

#[function_component]
pub fn Channel(props: &ChannelProps) -> HtmlResult {
    let result = use_future_with_deps(
        |url| async move {
            tauri::invoke::<_, rss::Channel>("fetch_feed", &FetchChannelArgs { url: &url }).await
        },
        props.url.clone(),
    )?;

    let Ok(channel) = &*result else {
        log::error!("Failed to fetch channel {:?}", result.as_ref().unwrap_err());
        return Ok(html! { <>{"failed to fetch channel"}{result.as_ref().unwrap_err().to_string()}</> });
    };

    let items: Html = channel
        .items
        .iter()
        .enumerate()
        .map(|(id, item)| {
            let description = item
                .content()
                .or(item.description())
                .map(|desc| desc.lines().next())
                .flatten()
                .map(ToString::to_string);

            html! {
                <ItemPreview
                    {id}
                    channel_url={props.url.clone()}
                    channel_title={channel.title.clone()}
                    title={item.title.clone()}
                    pub_date={item.pub_date.clone()}
                    description={description}
                />
            }
        })
        .collect();

    Ok(html! {
        <div class="channel">
            <strong class="channel-title">{&channel.title}</strong>
            <ul>
                {items}
            </ul>
        </div>
    })
}
