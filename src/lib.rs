use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::mission_pages::{
    CoopMissionListPage, CoopMissionPage, SoloMissionListPage, SoloMissionPage,
};
use pages::random_page::{CoopRandomPage, SoloRandomPage};
use std::sync::LazyLock;

mod components;
mod pages;

use pages::splash_page::SplashPage;
use serde::Deserialize;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router base="/beerio">
        <Routes fallback=|| SplashPage>
          <Route path=path!("/") view=SplashPage />
          <Route path=path!("solo") view=SoloRandomPage />
          <Route path=path!("coop") view=CoopRandomPage />
          <Route path=path!("solo/missions") view=SoloMissionListPage />
          <Route path=path!("solo/missions/:id") view=SoloMissionPage />
          <Route path=path!("coop/missions") view=CoopMissionListPage />
          <Route path=path!("coop/missions/:id") view=CoopMissionPage />
        </Routes>
      </Router>
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mission {
    name: String,
    description: String,
    #[serde(default)]
    details: Vec<String>,
    #[serde(default, rename = "200cc_adjustment")]
    two_hundred_cc_adjustment: String,
    #[serde(default)] // Makes this optional
    needs_random_item: bool,
    #[serde(default)] // Makes this optional
    needs_random_loadout: bool,
    #[serde(default)] // Makes this optional
    needs_random_number: usize,
    #[serde(default, rename = "all_items")]
    needs_item_checklist: bool,
    #[serde(default)]
    needs_coop_singles: bool,
}

#[derive(Debug, Deserialize)]
struct Missions {
    #[serde(rename = "missions")]
    solo_missions: Vec<Mission>,
    #[serde(rename = "coop_granprix")]
    coop_missions: Vec<Mission>,
    #[serde(rename = "coop_single")]
    coop_single_missions: Vec<Mission>,
}

pub static SOLO_MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/missions.toml"
    )))
    .unwrap()
    .solo_missions
});
pub static COOP_MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/missions.toml"
    )))
    .unwrap()
    .coop_missions
});
pub static COOP_SINGLE_MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/missions.toml"
    )))
    .unwrap()
    .coop_single_missions
});
