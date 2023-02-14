#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use url::Url;
use rss::Channel;
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Tauri(#[from] tauri::api::Error),
    #[error(transparent)]
    Rss(#[from] rss::Error)
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
async fn fetch_channel(url: Url) -> Result<Channel, Error> {
    let request = HttpRequestBuilder::new("GET", url)?
        .response_type(ResponseType::Binary);

    let client = ClientBuilder::new().build()?;

    let content = client.send(request).await?.bytes().await?;

    let channel = Channel::read_from(&content.data[..])?;

  Ok(channel)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_channel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}