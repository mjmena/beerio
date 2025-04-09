use leptos::prelude::*;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::{components::mission::MissionView, MISSIONS};

#[component]
pub fn RandomItemDisplay(seed: Signal<[u8; 32]>) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(seed.get());
    let item_id = move || rng().random_range(0..ITEMS.len());
    view! {
        <div class="max-w-md text-center text-4xl p-4">
            <img src=move || format!("/assets/items/{}.png", ITEMS.get(item_id()).unwrap() ) />
        </div>
    }
}

#[component]
pub fn RandomMissionDisplay(seed: Signal<[u8; 32]>, player: Signal<usize>) -> impl IntoView {
    let mission_ids = move || generate_numbers_from_hash(seed.get(), 12, 0, MISSIONS.len() - 1);
    let mission = Signal::derive(move || {
        let mission_id = *mission_ids().get(player.get() - 1).unwrap();
        MISSIONS.get(mission_id).unwrap().clone()
    });
    view! {
        <MissionView mission/>
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
