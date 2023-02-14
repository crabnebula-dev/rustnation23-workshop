use yew::prelude::*;

#[function_component]
pub fn ItemPreview() -> Html {
    html! {
        <li class="item-preview">
            {"pub date"}
            <strong>
                {"title"}
            </strong>
            <span class="item-preview-description">
                {"short description"}
            </span>
        </li>
    }
}