use ammonia::{Builder as AmmoniaBuilder, UrlRelative};
use serde::Serialize;
use tauri_sys::tauri;
use url::Url;
use yew::{prelude::*, suspense::use_future_with_deps};

use crate::components::date::Date;

#[derive(Debug, Properties, PartialEq)]
pub struct ItemProps {
    pub url: Url,
    pub id: usize,
}

#[derive(Serialize)]
struct FetchChannelItemArgs<'a> {
    url: &'a Url,
    id: usize,
}

#[function_component]
pub fn Item(props: &ItemProps) -> HtmlResult {
    let result = use_future_with_deps(
        |deps| async move {
            tauri::invoke::<_, rss::Item>(
                "fetch_channel_item",
                &FetchChannelItemArgs {
                    url: &deps.0,
                    id: deps.1,
                },
            )
            .await
        },
        (props.url.clone(), props.id),
    )?;

    let Ok(item) = &*result else {
        return Ok(html!{ "failed to fetch channel item" })
    };

    let authors = item
        .author()
        .map(|author| html! { author })
        .or(item
            .dublin_core_ext()
            .map(|dc| html! { dc.creators.iter().collect::<Html>() }))
        .unwrap_or_default();

    let safe_content = item
        .content()
        .or(item.description())
        .map(|input| {
            let base_url = Url::parse(&props.url.origin().unicode_serialization()).unwrap();

            AmmoniaBuilder::new()
                .url_relative(UrlRelative::RewriteWithBase(base_url))
                .set_tag_attribute_value("a", "target", "_blank")
                .clean(input)
                .to_string()
        })
        .unwrap_or_default();

    Ok(html! {
        <>
            <header>
                if let Some(pub_date) = &item.pub_date {
                    <Date date={pub_date.clone()} />
                }
                if let Some(title) = &item.title {
                    <h1>{title}</h1>
                }
                <small>{authors}</small>
            </header>
            <article>
                {Html::from_html_unchecked(AttrValue::from(safe_content))}
            </article>
        </>
    })
}
