use std::sync::OnceLock;

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
    let result = RULES.set(
        toml::from_str::<Rules>(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/rules.toml"
        )))
        .unwrap()
        .rules,
    );

    result.unwrap();

    view! {
        <Router base="/beerio".to_string() >
            <Routes fallback=SeedForm >
                <Route path=path!("/") view=SeedForm/>
               <Route path=path!(":seed") view=SeedDisplay/>
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

pub static RULES: OnceLock<Vec<Rule>> = std::sync::OnceLock::new();
