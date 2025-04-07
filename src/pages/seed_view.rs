use leptos::prelude::*;
use leptos_router::{
    components::A,
    hooks::{use_params_map, use_query_map},
};

use hex;
use rand::{SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::{
    MISSIONS,
    components::{css::Button, layout::Navbar, mission::MissionView},
};

#[component]
pub fn SeedView() -> impl IntoView {
    //collect url /:seed
    let params = use_params_map();
    let seed = move || params.read().get("seed").unwrap_or_default();

    //collect query info ?player= &round=
    let query = use_query_map();
    let player = move || {
        query
            .read()
            .get("player")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1)
    };

    let round = move || {
        query
            .read()
            .get("round")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1)
    };

    let hash = move || generate_seed_hash(seed().clone(), round());

    let rules = move || generate_numbers_from_hash(hash(), 12, 0, MISSIONS.len() - 1);
    let rule = move || {
        *rules()
            .get(player() - 1)
            .expect("rules vector should be generated now")
    };

    let mission = Signal::derive(move || MISSIONS.get(rule()).unwrap().clone());

    view! {
        <div class="min-h-screen h-screen bg-gray-100 flex flex-col items-center">
            <h1 class="text-2xl font-bold text-gray-800 text-center mb-4">{move || format!("Round {}", round())}</h1>
            <MissionView mission/>
            <div class="absolute bottom-20 flex flex-row w-full max-w-md gap-x-4 items-end justify-center">
                <Show when=move || 1 < round() >
                    <div class="w-1/2">
                    <A href=move || format!("?player={}&round={}", player(), round()-1) >
                        <Button>PAST ROUND</Button>
                    </A>
                    </div>
                </Show>
                <div class="w-1/2">
                <A href=move ||  format!("?player={}&round={}", player(), round()+1)>
                    <Button>NEXT ROUND </Button>
                </A>
                </div>
            </div>
        </div>
        <Navbar />

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

fn generate_numbers_from_hash(seed: [u8; 32], count: usize, min: usize, max: usize) -> Vec<usize> {
    // Create a stable seed from the input string
    // Use ChaCha20 RNG - guaranteed portable and stable
    let mut rng = ChaCha20Rng::from_seed(seed);

    // Generate and shuffle the range
    let mut numbers: Vec<usize> = (min..=max).collect();
    numbers.shuffle(&mut rng);

    numbers.into_iter().take(count).collect()
}
