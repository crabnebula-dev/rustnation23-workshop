use yew::prelude::*;

#[function_component]
pub fn ChannelList() -> Html {
    html! {
        <aside>
            <ul class="channel-list">
                {"chs"}
            </ul>
        </aside>
    }
}