use serde::Serialize;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use yew::{prelude::*, suspense::use_future_with_deps};
use url::Url;

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
            let raw_args = serde_wasm_bindgen::to_value(
                &FetchChannelArgs { url: &url }
            )?;
            let raw_result = invoke("fetch_channel", raw_args).await?;
            serde_wasm_bindgen::from_value::<rss::Channel>(raw_result)
        },
        props.url.clone(),
    )?;

    let Ok(channel) = &*result else {
        return Ok(html! { "Failed to fetch channel" });
    };

    Ok(html! {
        <aside class="channel">
            <strong class="channel-title">{&channel.title}</strong>
            <ul>
                {"items"}
            </ul>
        </aside>
    })
}