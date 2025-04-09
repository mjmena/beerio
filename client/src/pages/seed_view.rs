use leptos::prelude::*;
use leptos_router::{
    components::A,
    hooks::{use_params_map, use_query_map},
};

use hex;
use sha2::{Digest, Sha256};

use crate::components::{
    css::Button,
    random::{RandomItemDisplay, RandomMissionDisplay},
};

#[component]
pub fn SeedView() -> impl IntoView {
    //collect url /:seed
    let params = use_params_map();
    let seed = move || params.read().get("seed").unwrap_or_default();

    //collect query info ?player= &round=
    let query = use_query_map();
    let player = Signal::derive(move || {
        query
            .read()
            .get("player")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1)
    });

    let round = move || {
        query
            .read()
            .get("round")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1)
    };

    let seed = Signal::derive(move || generate_seed_hash(seed().clone(), round()));

    view! {
        <div class="min-h-screen h-screen bg-gray-100 flex flex-col items-center">
            <h1 class="text-2xl font-bold text-gray-800 text-center mb-4">{move || format!("Round {}", round())}</h1>
            <RandomMissionDisplay seed player/>
            <RandomItemDisplay seed/>
            <div class="absolute bottom-20 flex flex-row w-full max-w-md gap-x-4 items-end justify-center">
                <Show when=move || 1 < round() >
                    <div class="w-1/2">
                    <A href=move || format!("?player={}&round={}", player.get(), round()-1) >
                        <Button>PAST ROUND</Button>
                    </A>
                    </div>
                </Show>
                <div class="w-1/2">
                <A href=move ||  format!("?player={}&round={}", player.get(), round()+1)>
                    <Button>NEXT ROUND </Button>
                </A>
                </div>
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
