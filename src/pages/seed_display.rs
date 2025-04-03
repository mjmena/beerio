use leptos::prelude::*;
use leptos_router::{
    components::A,
    hooks::{use_params_map, use_query_map},
};

use rand::{SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::components::{css::Button, rule_display::RuleDisplay};

#[component]
pub fn SeedDisplay() -> impl IntoView {
    let params = use_params_map();
    let player = Signal::derive(move || {
        params
            .read()
            .get("player")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or_default()
    });
    let seed = Signal::derive(move || params.read().get("seed").unwrap_or_default());

    let query = use_query_map();
    let round = query
        .read()
        .get("round")
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap_or_default();

    let rules = Memo::new(move |_| generate_numbers_from_seed(&seed.get(), 12, 0, 12));
    let rule = Signal::derive(move || {
        *rules
            .get()
            .get(player.get())
            .expect("rules vector is generated")
    });
    view! {
        <div class="min-h-screen bg-gray-100 flex flex-col items-center">
            <RuleDisplay rule/>
            <div class="w-full max-w-md p-6">
            <A href=move || format!("/?seed={}&player={}", seed.get(), player.get()) >
                <Button>BACK</Button>
            </A>
            <A href=move || format!("/?seed={}&player={}", seed.get(), player.get()) >
                <Button>NEXT</Button>
            </A>
            </div>
        </div>

    }
}

fn generate_numbers_from_seed(seed: &str, count: usize, min: u32, max: u32) -> Vec<u32> {
    // Create a stable seed from the input string
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    let hash = hasher.finalize();

    // Use first 32 bytes for ChaCha20 (requires exactly 32 bytes)
    let mut seed_bytes = [0u8; 32];
    seed_bytes.copy_from_slice(&hash[..32]);

    // Use ChaCha20 RNG - guaranteed portable and stable
    let mut rng = ChaCha20Rng::from_seed(seed_bytes);

    // Generate and shuffle the range
    let mut numbers: Vec<u32> = (min..=max).collect();
    numbers.shuffle(&mut rng);

    numbers.into_iter().take(count).collect()
}
