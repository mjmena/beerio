use leptos::prelude::*;
use leptos_router::{
    components::A,
    hooks::{use_params_map, use_query_map},
};

use hex;
use sha2::{Digest, Sha256};

use crate::components::random::RandomMissionDisplay;

#[component]
pub fn SeedView() -> impl IntoView {
    //collect url /:seed
    let params = use_params_map();

    let get_seed = move || params.read().get("seed").unwrap_or_default();
    let get_player = Signal::derive(move || {
        params
            .read()
            .get("player")
            .unwrap_or_default()
            .parse()
            .unwrap_or(1)
    });

    let get_round = || {
        use_query_map()
            .read()
            .get("round")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1)
    };

    let get_seed = Signal::derive(move || generate_seed_hash(get_seed(), get_round()));

    let button_css = "text-center w-full h-full px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-medium rounded-md shadow-sm text-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 transition-colors";

    view! {
      <div class="flex flex-col items-center h-screen min-h-screen bg-gray-100">
        <h1 class="mb-4 text-2xl font-bold text-center text-gray-800">
          {move || format!("Round {}", get_round())}
        </h1>
        <RandomMissionDisplay get_seed get_player />
        <div class="flex absolute bottom-20 flex-row gap-x-4 justify-center items-end w-full max-w-md">
          <Show when=move || 1 < get_round()>
            <A
              href=move || format!("?round={}", get_round().checked_sub(1).unwrap_or(1))
              attr:class=button_css
            >
              Previous Round
            </A>
          </Show>
          <A href=move || format!("?round={}", get_round() + 1) attr:class=button_css>
            Next Round
          </A>
        </div>
      </div>
    }
}

fn generate_seed_hash(seed: String, iterations: usize) -> [u8; 32] {
    let hash = (0..iterations).fold(seed, |seed, _| {
        let mut hasher = Sha256::new();

        // Write input message
        hasher.update(seed.as_bytes());

        // Read hash digest and consume hasher
        let result = hasher.finalize();

        // Convert to hex string
        hex::encode(result)
    });

    let mut seed_bytes = [0u8; 32];
    seed_bytes.copy_from_slice(&hash.as_bytes()[..32]);
    seed_bytes
}
