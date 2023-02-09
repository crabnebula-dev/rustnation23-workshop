use serde::Serialize;
use tauri_sys::tauri;
use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize)]
struct AddChannelArgs<'a> {
    url: &'a Url,
}

#[function_component]
pub fn AddChannel() -> Html {
    let input_node_ref = use_node_ref();

    let url_handle = use_state(String::default);
    let url = (*url_handle).clone();

    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                url_handle.set(input.value());
            }
        })
    };

    let onclick = {
        let url = url.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let url = Url::parse(&url).unwrap();

            wasm_bindgen_futures::spawn_local(async move {
                tauri::invoke::<_, ()>("add_channel", &AddChannelArgs { url: &url })
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <div class="add-channel">
            <div class="titlebar" data-tauri-drag-region="true"></div>

            <form action="" class="add-channel-form">
                <label for="my-input">
                    { "My input:" }
                    <input ref={input_node_ref}
                        {onchange}
                        type="url"
                        value={url}
                    />
                </label>
                <button type="submit" {onclick}>{"Add Channel"}</button>
            </form>
        </div>
    }
}
