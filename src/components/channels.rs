use tauri_sys::tauri;
use url::Url;
use yew::{prelude::*, suspense::use_future};

use crate::components::{channel_preview::ChannelPreview, add_channel::AddChannel};

#[function_component]
pub fn Channels() -> HtmlResult {
    let result = use_future(|| async { tauri::invoke::<_, Vec<Url>>("get_channels", &()).await })?;

    let Ok(channels) = &*result else {
        log::error!("Failed to get channels {:?}", result.as_ref().unwrap_err());
        return Ok(html! { <>{"failed to get channels"}{result.as_ref().unwrap_err().to_string()}</> });
    };

    let channel_previews: Html = channels
        .iter()
        .map(|url| {
            html! {
                <Suspense fallback={html! {"loading channel preview"}}>
                    <ChannelPreview url={url.clone()} />
                </Suspense>
            }
        })
        .collect();

    Ok(html! {
        <ul class="channels">
            {channel_previews}
            <AddChannel />
        </ul>
    })
}
