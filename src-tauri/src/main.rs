#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod error;
mod cache;

use cached::{proc_macro::cached, ExpiringValueCache};
use rss::Channel;
use tauri::{
    api::http::{ClientBuilder, HttpRequestBuilder, ResponseType},
    AppHandle, Runtime, State,
};
use tauri_plugin_store::{with_store, JsonValue, StoreCollection};
use url::Url;
use error::Error;
use cache::ChannelWithExpiry;

#[tauri::command]
fn add_channel<R: Runtime>(
    app_handle: AppHandle<R>,
    collection: State<StoreCollection<R>>,
    url: Url,
) -> Result<(), Error> {
    with_store(app_handle, collection, "./channels.bin", |store| {
        store.insert(url.to_string(), JsonValue::Null)
    })
    .map_err(Into::into)
}

#[tauri::command]
fn get_channels<R: Runtime>(
    app_handle: AppHandle<R>,
    collection: State<StoreCollection<R>>,
) -> Result<Vec<Url>, Error> {
    with_store(app_handle, collection, "./channels.bin", |store| {
        let urls = store
            .keys()
            .map(|url| Url::parse(url).expect("File contained invalid URL"))
            .collect();

        Ok(urls)
    })
    .map_err(Into::into)
}

#[tauri::command]
async fn fetch_channel_item(url: Url, id: usize) -> Result<rss::Item, Error> {
    let channel = fetch_feed(url).await?;

    Ok(channel.inner.items()[id].clone())
}

#[tauri::command]
#[cached(
    type = "ExpiringValueCache<Url, ChannelWithExpiry>",
    create = "{ ExpiringValueCache::with_size(100) }",
    result = true
)]
async fn fetch_feed(url: Url) -> Result<ChannelWithExpiry, Error> {
    let client = ClientBuilder::new().build().unwrap();
    let content = client
        .send(
            HttpRequestBuilder::new("GET", url)
                .unwrap()
                .response_type(ResponseType::Binary),
        )
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content.data[..])?;

    Ok(ChannelWithExpiry::from(channel))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_channel,
            get_channels,
            fetch_feed,
            fetch_channel_item
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
