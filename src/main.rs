use leptos::logging;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (seed, set_seed) = create_signal(String::new());
    let (player, set_player) = create_signal(1);

    let on_submit = move |_| {
        logging::log!(
            "Joining as player {} with seed: {}",
            player.get(),
            seed.get()
        );
    };

    view! {
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
                        on:input=move |ev| set_seed.set(event_target_value(&ev))
                        prop:value=move || seed.get()
                    />
                </div>
            </div>

            {/* Player selection grid (unchanged) */}
            <div class="flex-grow w-full p-4 grid grid-cols-3 gap-3 auto-rows-fr max-w-4xl">
                {(1..=12).map(|player_num| {
                    let is_selected = move || player.get() == player_num;
                    view! {
                        <button
                            type="button"
                            class=move || format!(
                                "flex items-center justify-center text-4xl font-bold rounded-xl transition-all duration-200 {}",
                                if is_selected() {
                                    "bg-blue-600 text-white shadow-lg transform scale-[1.03]"
                                } else {
                                    "bg-white text-gray-800 border border-gray-300 hover:bg-gray-50"
                                }
                            )
                            on:click=move |_| set_player.set(player_num)
                        >
                            {player_num}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            {/* Normal-sized centered submit button */}
            <div class="w-full max-w-md p-6">
                <button
                    on:click=on_submit
                    class="w-full px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-medium
                           rounded-md shadow-sm text-lg focus:outline-none focus:ring-2 focus:ring-offset-2 
                           focus:ring-green-500 transition-colors"
                >
                    JOIN AS PLAYER {move || player.get()}
                </button>
            </div>
        </div>
    }
}
