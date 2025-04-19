use leptos::prelude::*;

use leptos_icons::Icon;
use leptos_router::hooks::use_query_map;
use rand::{Rng, SeedableRng, distr::Alphanumeric, rng, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::{
    COOP_MISSIONS, COOP_SINGLE_MISSIONS, SOLO_MISSIONS,
    components::{layout::Layout, mission::MissionView},
};

#[component]
pub fn SoloRandomPage() -> impl IntoView {
    let get_seed_query = || use_query_map().read().get("seed").unwrap_or_default();
    let get_seed = Signal::derive(move || string_to_sha256(&get_seed_query()));

    view! {
      <Layout>
        <div class="flex flex-col justify-evenly bg-transparent h-9/10">
          <RandomSoloMissionDisplay get_seed get_player=|| 1 />
          <RandomSoloMissionDisplay get_seed get_player=|| 2 />
        </div>
      </Layout>
    }
}

#[component]
pub fn CoopRandomPage() -> impl IntoView {
    let get_seed_query = || use_query_map().read().get("seed").unwrap_or_default();
    let get_seed = Signal::derive(move || string_to_sha256(&get_seed_query()));

    view! {
      <Layout>
        <div class="flex flex-col bg-transparent h-9/10">
          <RandomCoopMissionDisplay get_seed get_player=|| 1 />
        </div>
      </Layout>
    }
}

#[component]
pub fn RandomItemDisplay(get_seed: Signal<[u8; 32]>) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(get_seed());
    let item_id = move || rng().random_range(0..ITEMS.len());
    view! {
      <img
        class="object-contain size-40"
        src=move || { format!("assets/items/{}.png", ITEMS.get(item_id()).unwrap()) }
      />
    }
}

#[component]
pub fn RandomLoadoutDisplay(get_seed: Signal<[u8; 32]>) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(get_seed());
    let get_character = move || {
        let id = rng().random_range(0..CHARACTERS.len());
        CHARACTERS.get(id).unwrap().to_string()
    };
    let get_kart = move || {
        let id = rng().random_range(0..KARTS.len());
        KARTS.get(id).unwrap().to_string()
    };
    let get_wheel = move || {
        let id = rng().random_range(0..WHEELS.len());
        WHEELS.get(id).unwrap().to_string()
    };
    let get_glider = move || {
        let id = rng().random_range(0..GLIDERS.len());
        GLIDERS.get(id).unwrap().to_string()
    };
    view! {
      <div class="flex flex-wrap justify-center p-4">
        <div class="flex flex-col justify-end size-40">
          <img
            src=move || {
              format!("assets/characters/{}.webp", get_character().to_lowercase().replace(" ", "_"))
            }
            class="object-scale-down"
          />
          {move || get_character().to_string()}
        </div>
        <div class="flex flex-col justify-end size-40">
          <img
            src=move || format!("assets/karts/{}.webp", get_kart().to_lowercase().replace(" ", "_"))
            class="object-scale-down"
          />
          {move || get_kart()}
        </div>
        <div class="flex flex-col justify-end size-40">
          <img
            src=move || {
              format!("assets/wheels/{}.webp", get_wheel().to_lowercase().replace(" ", "_"))
            }
            class="object-scale-down"
          />
          {move || get_wheel().to_string()}
        </div>
        <div class="flex flex-col justify-end size-40">
          <img
            src=move || {
              format!("assets/gliders/{}.webp", get_glider().to_lowercase().replace(" ", "_"))
            }
            class="object-scale-down"
          />
          {move || get_glider().to_string()}
        </div>
      </div>
    }
}

#[component]
pub fn RandomCoopSingleMissionDisplay(get_seed: Signal<[u8; 32]>) -> impl IntoView {
    let (get_seed, set_seed) = signal(get_seed());

    let mission_ids =
        move || generate_numbers_from_hash(get_seed(), 12, 0, COOP_SINGLE_MISSIONS.len() - 1);
    let get_mission = Signal::derive(move || {
        let mission_id = *mission_ids().get(1).unwrap();
        COOP_SINGLE_MISSIONS.get(mission_id).unwrap().clone()
    });

    view! {
      <div class="flex flex-col items-center p-4 m-6 border border-red-100">
        <MissionView get_mission get_seed />
        <button
          class="py-2 px-6 text-white bg-red-500 rounded-full"
          on:click=move |_| set_seed(string_to_sha256(&hex::encode(get_seed())))
        >
          Next Single Mission
        </button>
      </div>
    }
}
#[component]
pub fn RandomCoopMissionDisplay(
    get_seed: Signal<[u8; 32]>,
    get_player: impl Fn() -> usize + Send + Sync + 'static,
) -> impl IntoView {
    let mission_ids =
        move || generate_numbers_from_hash(get_seed(), 12, 0, COOP_MISSIONS.len() - 1);
    let get_mission = Signal::derive(move || {
        let mission_id = *mission_ids().get(get_player() - 1).unwrap();
        COOP_MISSIONS.get(mission_id).unwrap().clone()
    });
    view! {
      <div class="flex flex-col items-center">
        <MissionView get_mission get_seed />
      </div>
    }
}
#[component]
pub fn RandomSoloMissionDisplay(
    get_seed: Signal<[u8; 32]>,
    get_player: impl Fn() -> usize + Send + Sync + 'static,
) -> impl IntoView {
    let mission_ids =
        move || generate_numbers_from_hash(get_seed(), 12, 0, SOLO_MISSIONS.len() - 1);
    let get_mission = Signal::derive(move || {
        let mission_id = *mission_ids().get(get_player() - 1).unwrap();
        SOLO_MISSIONS.get(mission_id).unwrap().clone()
    });
    view! {
      <div class="flex flex-col items-center">
        <MissionView get_mission get_seed />
      </div>
    }
}

#[component]
pub fn RandomNumberDisplay(get_seed: Signal<[u8; 32]>, number: usize) -> impl IntoView {
    let rng = move || ChaCha20Rng::from_seed(get_seed());
    let get_number = move || rng().random_range(0..number);

    let icon = move || match get_number() + 1 {
        1 => icondata::LuDice1,
        2 => icondata::LuDice2,
        3 => icondata::LuDice3,
        4 => icondata::LuDice4,
        5 => icondata::LuDice5,
        6 => icondata::LuDice6,
        _ => icondata::LuDices,
    };
    view! { <Icon icon=icon() attr:class="size-40" /> }
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
    "coin",
    "banana",
    "triple_banana",
    "green_shell",
    "triple_green_shells",
    "red_shell",
    "triple_red_shells",
    "mushroom",
    "triple_mushrooms",
    "golden_mushroom",
    "super_star",
    "lightning",
    "bob-omb",
    "boo",
    "fire_flower",
    "boomerang_flower",
    "piranha_plant",
    "bullet_bill",
    "spiny_shell",
    "super_horn",
    "blooper",
    "crazy_eight",
];
pub static KARTS: [&str; 36] = [
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
    // "Gold Standard",
];
pub static WHEELS: [&str; 21] = [
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
    "GLA Tires",
    "Triforce Tires",
    "Leaf Tires",
    "Ancient Tires",
    // "Gold Tires",
];
pub static GLIDERS: [&str; 14] = [
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
    "Hylian Kite",
    "Paper Glider",
    "Paraglider",
    // "Gold Glider",
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

pub fn generate_random_string() -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}

pub fn string_to_sha256(input: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}
