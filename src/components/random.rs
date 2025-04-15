use leptos::prelude::*;
use rand::{Rng, SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;

use crate::{MISSIONS, components::mission::MissionView};

#[component]
pub fn RandomItemDisplay(get_seed: Signal<[u8; 32]>) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(get_seed());
    let item_id = move || rng().random_range(0..ITEMS.len());
    view! {
      <div class="p-4">
        <img
          class="object-contain size-40"
          src=move || format!("/assets/items/{}.png", ITEMS.get(item_id()).unwrap())
        />
      </div>
    }
}

#[component]
pub fn RandomLoadoutDisplay(get_seed: Signal<[u8; 32]>) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(get_seed());
    let get_kart_id = move || rng().random_range(0..KARTS.len());
    let get_wheel_id = move || rng().random_range(0..WHEELS.len());
    let get_glider_id = move || rng().random_range(0..GLIDERS.len());
    view! {
      <div class="p-4">
        <div>{move || KARTS.get(get_kart_id()).unwrap().to_string()}</div>
        <div>{move || WHEELS.get(get_wheel_id()).unwrap().to_string()}</div>
        <div>{move || GLIDERS.get(get_glider_id()).unwrap().to_string()}</div>
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
      <MissionView mission />
      <Show when=move || mission().needs_random_item>
        <RandomItemDisplay get_seed />
      </Show>
    }
}

#[component]
pub fn RandomNumberDisplay(get_seed: Signal<[u8; 32]>, number: usize) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(get_seed());
    let get_number = move || rng().random_range(0..number);

    view! { <div>{move || get_number() + 1}</div> }
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
pub static KARTS: [&str; 37] = [
    "Standard Kart",
    "Pipe Frame",
    "B Dasher",
    "Mach 8",
    "Steel Driver",
    "Cat Cruiser",
    "Circuit Special",
    "Tri-Speeder",
    "Badwagon",
    "Prancer",
    "Biddybuggy",
    "Landship",
    "Sneeker",
    "Sports Coupe",
    "Gold Standard",
    "GLA",
    "W 25 Silver Arrow",
    "300 SL Roadster",
    "Blue Falcon",
    "Tanooki Kart",
    "Bone Rattler",
    "Inkstriker",
    "Master Cycle",
    "Streetle",
    "P-Wing",
    "Koopa Clown",
    "Standard Bike",
    "Comet",
    "Sport Bike",
    "The Duke",
    "Flame Rider",
    "Varmint",
    "Mr. Scooty",
    "Jet Bike",
    "Yoshi Bike",
    "Master Cycle Zero",
    "City Tripper",
];
pub static WHEELS: [&str; 22] = [
    "Standard",
    "Monster",
    "Roller",
    "Slim",
    "Slick",
    "Metal",
    "Button",
    "Off-Road",
    "Sponge",
    "Wood",
    "Cushion",
    "Blue Standard",
    "Hot Monster",
    "Azure Roller",
    "Crimson Slim",
    "Cyber Slick",
    "Retro Off-Road",
    "Gold Tires",
    "GLA Tires",
    "Triforce Tires",
    "Leaf Tires",
    "Ancient Tires",
];
pub static GLIDERS: [&str; 15] = [
    "Super Glider",
    "Cloud Glider",
    "Wario Wing",
    "Waddle Wing",
    "Peach Parasol",
    "Parachute",
    "Parafoil",
    "Flower Glider",
    "Bowser Kite",
    "Plane Glider",
    "MKTV Parafoil",
    "Gold Glider",
    "Hylian Kite",
    "Paper Glider",
    "Paraglider",
];

pub static CHARACTERS: [&str; 47] = [
    "Baby Daisy",
    "Baby Luigi",
    "Baby Mario",
    "Baby Peach",
    "Baby Rosalina",
    "Birdo",
    "Cat Peach",
    "Dry Bones",
    "Lemmy",
    "Bowser Jr.",
    "Daisy",
    "Diddy Kong",
    "Iggy",
    "Inkling Boy",
    "Inkling Girl",
    "Isabelle",
    "Kamek",
    "Koopa Troopa",
    "Lakitu",
    "Larry",
    "Link",
    "Luigi",
    "Ludwig",
    "Mario",
    "Morton",
    "Pauline",
    "Peach",
    "Peachette",
    "Rosalina",
    "Roy",
    "Shy Guy",
    "Toad",
    "Toadette",
    "Villager",
    "Wendy",
    "Wiggler",
    "Yoshi",
    "Bowser",
    "Donkey Kong",
    "Dry Bowser",
    "Funky Kong",
    "King Boo",
    "Metal Mario",
    "Petey Piranha",
    "Pink Gold Peach",
    "Wario",
    "Waluigi",
];
