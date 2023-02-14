use yew::prelude::*;

#[function_component]
pub fn Item() -> Html {
    html! {
        <main>
            <header>
                {"pub date"}
                <h1>{"title"}</h1>
            </header>
            <article>
                {"content"}
            </article>
        </main>
    }
}
