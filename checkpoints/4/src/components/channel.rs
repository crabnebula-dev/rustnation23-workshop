use serde::Serialize;
use url::Url;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::{prelude::*, suspense::use_future_with_deps};

use crate::components::item_preview::ItemPreview;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
struct FetchChannelArgs<'a> {
    url: &'a Url,
}

#[derive(Properties, PartialEq)]
pub struct ChannelProps {
    pub url: Url,
}

#[function_component]
pub fn Channel(props: &ChannelProps) -> HtmlResult {
    let result = use_future_with_deps(
        |url| async move {
            let raw_args = serde_wasm_bindgen::to_value(&FetchChannelArgs { url: &url })?;
            let raw_result = invoke("fetch_channel", raw_args).await?;
            serde_wasm_bindgen::from_value::<rss::Channel>(raw_result)
        },
        props.url.clone(),
    )?;

    let Ok(channel) = &*result else {
        return Ok(html! { "Failed to fetch channel" });
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
