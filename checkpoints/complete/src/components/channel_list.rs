use tauri_sys::tauri;
use url::Url;
use yew::{prelude::*, suspense::use_future};

use crate::components::{add_channel::AddChannel, channel_preview::ChannelPreview};

#[function_component]
pub fn ChannelList() -> HtmlResult {
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
        <aside>
            <ul class="channel-list">
                {channel_previews}
                <AddChannel />
            </ul>
        </aside>
    })
}
