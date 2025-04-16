use leptos::prelude::*;

use leptos_router::hooks::{use_navigate, use_query_map};
use rand::{Rng, distr::Alphanumeric, rng};
use sha2::{Digest, Sha256};

use crate::components::random::{RandomItemDisplay, RandomLoadoutDisplay, RandomMissionDisplay};

#[component]
pub fn RandomPage() -> impl IntoView {
    let get_seed = || use_query_map().read().get("seed").unwrap_or_default();

    Effect::new_sync(move || {
        if get_seed().is_empty() {
            use_navigate()(
                &format!("/?seed={}", generate_random_string(5)),
                Default::default(),
            );
        }
    });

    let get_seed = Signal::derive(move || string_to_sha256(&get_seed()));

    view! {
      <div class="grid grid-cols-2 h-screen min-h-screen bg-gray-100">
        <RandomMissionDisplay get_seed get_player=|| 1 />
        <RandomMissionDisplay get_seed get_player=|| 2 />
        <RandomItemDisplay get_seed />
        <RandomLoadoutDisplay get_seed />
      </div>
    }
}

fn generate_random_string(length: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn string_to_sha256(input: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}
