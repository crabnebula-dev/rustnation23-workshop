use serde::Serialize;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct DateProps {
    pub date: String,
}

#[derive(Serialize)]
struct FormatDateOptions<'a> {
    weekday: &'a str,
    year: &'a str,
    month: &'a str,
    day: &'a str,
}

#[function_component]
pub fn Date(props: &DateProps) -> Html {
    let pub_date = js_sys::Date::new(&JsValue::from_str(&props.date));

    let date_format_options = serde_wasm_bindgen::to_value(&FormatDateOptions {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
    })
    .unwrap();

    html! {
        <time datetime={pub_date.to_iso_string().as_string().unwrap()}>
            {pub_date.to_locale_date_string("en-US", &date_format_options)}
        </time>
    }
}
