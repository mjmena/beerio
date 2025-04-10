use leptos::prelude::*;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::{components::mission::MissionView, MISSIONS};

#[component]
pub fn RandomItemDisplay(get_seed: impl Fn() -> [u8; 32] + Send + Sync + 'static) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(get_seed());
    let item_id = move || rng().random_range(0..ITEMS.len());
    view! {
        <div class="p-4">
            <img class="size-40 object-contain" src=move || format!("/assets/items/{}.png", ITEMS.get(item_id()).unwrap() ) />
        </div>
    }
}

#[component]
pub fn RandomMissionDisplay(
    get_seed: Signal<[u8; 32]>,
    get_player: impl Fn() -> usize + Send + Sync + 'static,
) -> impl IntoView {
    let mission_ids = move || generate_numbers_from_hash(get_seed(), 12, 0, MISSIONS.len() - 1);
    let mission = Signal::derive(move || {
        let mission_id = *mission_ids().get(get_player() - 1).unwrap();
        MISSIONS.get(mission_id).unwrap().clone()
    });
    view! {
        <MissionView mission/>
        <Show when=move || mission().needs_random_item >
            <RandomItemDisplay get_seed />
        </Show>
    }
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

pub static ITEMS: [&str; 22] = [
    "banana",
    "blooper",
    "bob-omb",
    "boo",
    "boomerang_flower",
    "bullet_bill",
    "coin",
    "crazy_eight",
    "fire_flower",
    "golden_mushroom",
    "green_shell",
    "lightning",
    "mushroom",
    "piranha_plant",
    "red_shell",
    "spiny_shell",
    "super_horn",
    "super_star",
    "triple_banana",
    "triple_green_shells",
    "triple_mushrooms",
    "triple_red_shells",
];
