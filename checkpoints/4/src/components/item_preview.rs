use yew::prelude::*;
use url::Url;
use ammonia::Builder as AmmoniaBuilder;

#[derive(Debug, Properties, PartialEq)]
pub struct ItemPreviewProps {
    pub channel_url: Url,
    pub id: usize,
    pub channel_title: String,
    pub title: Option<String>,
    pub pub_date: Option<String>,
    pub description: Option<String>,
}

#[function_component]
pub fn ItemPreview(props: &ItemPreviewProps) -> Html {
    let safe_description = props
        .description
        .as_ref()
        .map(|input| AmmoniaBuilder::empty().clean(input).to_string())
        .unwrap_or_default();

    html! {
        <li key={props.id} class="item-preview">
            {"pub date"}
            if let Some(title) = &props.title {
                <strong>
                    {title}
                </strong>
            }
            <span class="item-preview-description">
                {&safe_description[0..100.min(safe_description.len())]}
            </span>
        </li>
    }
}
