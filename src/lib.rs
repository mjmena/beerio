use std::sync::LazyLock;

use components::layout::Navbar;
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

mod components;
mod pages;
use pages::{
    mission_pages::{MissionListPage, MissionPage},
    seed_form::SeedForm,
    seed_view::SeedView,
};
use serde::Deserialize;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/beerio".to_string() >
            <Routes fallback=|| "error">
                <ParentRoute path=path!("/") view = Navbar>
                    <Route path=path!("") view=SeedForm/>
                    <Route path=path!("missions") view=MissionListPage />
                    <Route path=path!("missions/:mission") view=MissionPage />
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
