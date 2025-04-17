use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use std::sync::LazyLock;

mod components;
mod pages;

use pages::{
    mission_pages::{MissionListPage, MissionPage},
    random_page::RandomPage,
    splash_page::SplashPage,
};
use serde::Deserialize;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router base="/beerio">
        <Routes fallback=|| "error">
          <Route path=path!("/") view=SplashPage />
          <Route path=path!("solo") view=RandomPage />
          <Route path=path!("missions") view=MissionListPage />
          <Route path=path!("missions/:mission") view=MissionPage />
        </Routes>
      </Router>
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mission {
    name: String,
    description: String,
    #[serde(default)] // Makes this optional
    needs_random_item: bool,
    // #[serde(default)] // Makes this optional
    // needs_random_kart: bool,
    // #[serde(default)] // Makes this optional
    // needs_random_wheel: bool,
    // #[serde(default)] // Makes this optional
    // needs_random_glider: bool,
}

#[derive(Debug, Deserialize)]
struct Missions {
    missions: Vec<Mission>,
}

pub static MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/missions.toml"
    )))
    .unwrap()
    .missions
});
