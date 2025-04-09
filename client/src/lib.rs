use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use std::sync::LazyLock;

use components::layout::Layout;

mod components;
mod pages;
use pages::{
    mission_pages::{MissionListPage, MissionPage},
    seed_form::SeedForm,
    seed_view::SeedView,
};
use serde::Deserialize;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
                <Stylesheet id="leptos" href="/pkg/beerio.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router >
            <Routes fallback=|| "error">
                <ParentRoute path=path!("") view=Layout>
                    <Route path=path!("") view=SeedForm/>
                    <ParentRoute path=path!("missions") view=MissionListPage >
                        <Route path=path!("") view=MissionPage />
                        <Route path=path!(":mission") view=MissionPage />
                    </ParentRoute>
                    <Route path=path!(":seed") view=SeedView />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mission {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Missions {
    #[serde(rename = "rules")]
    missions: Vec<Mission>,
}

pub static MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rules.toml"
    )))
    .unwrap()
    .missions
});
