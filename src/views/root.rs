use axum::response::Html;
use leptos::{
    view,
    IntoView,
    ssr::render_to_string,
};

use crate::components::meta::Metadata;

pub async fn root() -> Html<String> {
    let html = render_to_string(|cx| view! {
        cx,
        <Metadata />
        <img class="h-full w-full" src="/assets/pedro.gif"/>
    });

    return Html(html);
}