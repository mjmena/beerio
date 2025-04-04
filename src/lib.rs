use std::sync::LazyLock;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod pages;
use pages::{seed_display::SeedDisplay, seed_form::SeedForm};
use serde::Deserialize;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=SeedForm >
                <Route path=path!("/") view=SeedForm/>
                <Route path=path!("/:seed") view=SeedDisplay/>
            </Routes>
        </Router>
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Rules {
    #[serde(rename = "rules")]
    rules: Vec<Rule>,
}

pub static RULES: LazyLock<Vec<Rule>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Rules>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rules.toml"
    )))
    .unwrap()
    .rules
});
