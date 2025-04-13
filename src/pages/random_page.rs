use leptos::prelude::*;

use leptos_router::hooks::query_signal;
use rand::{distr::Alphanumeric, rng, Rng};
use sha2::{Digest, Sha256};

use crate::components::random::{RandomItemDisplay, RandomLoadoutDisplay, RandomMissionDisplay};

#[component]
pub fn RandomPage() -> impl IntoView {
    let (get_seed, set_seed) = query_signal::<String>("seed");

    Effect::new_sync(move || {
        if get_seed().is_none() {
            set_seed(Some(generate_random_string(5)));
        }
    });

    let get_seed = Signal::derive(move || string_to_sha256(&get_seed().unwrap_or_default()));

    view! {
        <div class="min-h-screen h-screen bg-gray-100 flex flex-col items-center">
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
