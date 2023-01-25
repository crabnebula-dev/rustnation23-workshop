use clap::Parser;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use ui::Route;
use yew::prelude::*;
use yew::ServerRenderer;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

const DEFAULT_TEMPLATE: &str = include_str!("../../index.html");

#[derive(Properties, PartialEq, Default)]
struct Props {
    route: Route,
}

#[function_component]
fn App(props: &Props) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(props.route.to_path());

    html! {
        <Router history={history}>
            <ui::App />
        </Router>
    }
}

#[derive(Debug, Parser)]
struct Opts {
    #[arg(long, short)]
    dist: PathBuf,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    env_logger::init();

    let template = tokio::fs::read_to_string(opts.dist.join("index.html"))
        .await
        .unwrap_or_else(|_| DEFAULT_TEMPLATE.to_string());
    let (index_html_before, index_html_after) = template.split_once("<body>").unwrap();
    let mut html_out = index_html_before.to_string();
    html_out.push_str("<body>");

    for route in Route::PRERENDER {
        let mut html_out = html_out.clone();

        let renderer = ServerRenderer::<App>::with_props(|| Props {
            route: route.clone(),
        });

        renderer.render_to_string(&mut html_out).await;
        html_out.push_str(index_html_after);

        let mut path = route
            .to_path()
            .strip_prefix('/')
            .map(ToString::to_string)
            .unwrap_or_default();
        if path.is_empty() || path.ends_with('/') {
            path.push_str("index")
        }
        path.push_str(".html");

        println!("writing to {:?}", opts.dist.join(&path));

        tokio::fs::File::create(opts.dist.join(&path))
            .await
            .unwrap()
            .write_all(html_out.as_bytes())
            .await
            .unwrap();
    }
}
