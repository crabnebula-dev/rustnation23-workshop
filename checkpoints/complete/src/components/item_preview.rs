use crate::{Route, components::date::Date};
use ammonia::Builder as AmmoniaBuilder;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

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
            <Link<Route> to={Route::Item { url: props.channel_url.clone(), id: props.id }}>
                if let Some(pub_date) = &props.pub_date {
                    <Date date={pub_date.clone()} />
                }
                if let Some(title) = &props.title {
                    <strong>
                        {title}
                    </strong>
                }
                <span class="item-preview-description">
                    {&safe_description[0..100.min(safe_description.len())]}
                </span>
            </Link<Route>>
        </li>
    }
}
