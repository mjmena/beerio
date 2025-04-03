use leptos::prelude::*;
use leptos_router::{components::Form, hooks::use_query_map};

#[component]
pub fn SeedForm() -> impl IntoView {
    let query = use_query_map();
    let player = RwSignal::new(
        query
            .read()
            .get("player")
            .unwrap_or_default()
            .parse()
            .unwrap_or(1),
    );
    let seed = RwSignal::new(query.read().get("seed").unwrap_or("seed".to_string()));

    view! {
        <Form method="GET" action= move || format!("/{}", seed.get())>

            <div class="min-h-screen bg-gray-100 flex flex-col items-center">
            {/* Centered container for seed input */}
            <div class="w-full max-w-md p-6">
                <h1 class="text-2xl font-bold text-gray-800 text-center mb-4">Choose Your Player</h1>

                {/* Normal-sized seed input */}
                <div class="mb-6">
                    <input
                        type="text"
                        class="w-full px-4 py-2 text-center border border-gray-300 rounded-md shadow-sm
                               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        placeholder="Enter game seed"
                        bind:value=seed
                    />
                </div>
            </div>

            {/* Player selection grid (unchanged) */}
            <div class="flex-grow w-full p-4 grid grid-cols-3 gap-3 auto-rows-fr max-w-4xl">
                {(1..=12).map(|player_num| {
                    let is_selected = move || player.get() == player_num;
                    view! {
                        <div class="relative h-full">
                            <input
                                type="radio"
                                id=format!("player_{}", player_num)
                                name="player"
                                value=player_num
                                class="absolute opacity-0 w-0 h-0"
                                checked=is_selected
                                on:change=move |_| player.set(player_num)
                            />
                            <label
                                for=format!("player_{}", player_num)
                                class=move || format!(
                                    "flex items-center justify-center h-full w-full text-4xl font-bold rounded-xl
                                    border-2 cursor-pointer transition-all duration-200 {}",
                                    if is_selected() {
                                        "bg-blue-600 text-white border-blue-700 shadow-lg transform scale-[1.03]"
                                    } else {
                                        "bg-white text-gray-800 border-gray-300 hover:bg-gray-50"
                                    }
                                )
                            >
                                {player_num}
                            </label>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            {/* Normal-sized centered submit button */}
            <div class="w-full max-w-md p-6">
                <button
                    type= "submit"
                    class="w-full px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-medium
                   rounded-md shadow-sm text-lg focus:outline-none focus:ring-2 focus:ring-offset-2 
                   focus:ring-green-500 transition-colors"
                >
                    Generate Seed
                </button>
            </div>
        </div>
        </Form>
    }
}
