use yew::prelude::*;

#[function_component]
pub fn Channel() -> Html {
    html! {
        <aside class="channel">
            <strong class="channel-title">{"title"}</strong>
            <ul>
                {"items"}
            </ul>
        </aside>
    }
}