use leptos::prelude::*;
use rand::{SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};
mod components;
use components::{rule::*, seed_form::*};

#[derive(Clone)]
enum State {
    Input,
    Output,
}

#[component]
pub fn App() -> impl IntoView {
    let state = RwSignal::new(State::Input);
    let seed = RwSignal::new(String::new());
    let player = RwSignal::new(0);

    let on_submit = move |_| state.set(State::Output);
    let rules = Memo::new(move |_| generate_numbers_from_seed(&seed.get(), 12, 0, 40));
    let rule = Signal::derive(move || {
        *rules
            .get()
            .get(player.get())
            .expect("rules vector is generated")
    });

    move || match state.get() {
        State::Input => view! {
            <SeedForm seed player submit_handler=on_submit />
        }
        .into_any(),
        State::Output => view! {
            <Rule rule on:click=move |_| state.set(State::Input) />
        }
        .into_any(),
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
