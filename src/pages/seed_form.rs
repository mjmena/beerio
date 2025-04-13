use leptos::prelude::*;
use leptos_router::{components::Form, hooks::query_signal};

#[component]
pub fn SeedForm() -> impl IntoView {
    let (get_seed, set_seed) = query_signal::<String>("seed");
    let (get_player, set_player) = query_signal::<usize>("player");

    view! {
        <Form method="GET" action=move || format!("{}/{}", get_seed().unwrap_or_default(), get_player().unwrap_or(1)) attr:class="flex flex-col flex-grow w-full items-center" >
            <div class="w-full max-w-md p-6">
                <h1 class="text-2xl font-bold text-gray-800 text-center mb-4">Enter Your Seed</h1>

                {/* Normal-sized seed input */}
                <div class="mb-6">
                    <input
                        type="text"
                        class="w-full px-4 py-2 text-center border border-gray-300 rounded-md shadow-sm
                               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        placeholder="Enter game seed"
                        prop:value= move|| get_seed().unwrap_or_default()
                        on:input=move|ev| {
                            let value = event_target_value(&ev);
                            set_seed(if value.is_empty(){None} else {Some(value)});
                        }
                    />
                </div>
               <h1 class="text-2xl font-bold text-gray-800 text-center mb-4">Choose Your Player</h1>
            </div>

            {/* Player selection grid (unchanged) */}
            <div class="flex-grow max-w-4xl w-full p-4 grid grid-cols-3 gap-3 auto-rows-fr">
                {(1..=12).map(|player_num| {
                    let is_selected = move || get_player().unwrap_or_default() == player_num;
                    view! {
                        <div class="relative h-full">
                            <input
                                type="radio"
                                id=format!("player_{}", player_num)
                                value=player_num
                                class="absolute opacity-0 w-0 h-0"
                                on:input=move|ev| {
                                    let value = event_target_value(&ev);
                                    set_player(if value.is_empty(){None} else {Some(value.parse().unwrap_or(1))});
                                }
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

            <input type="hidden" value="1" name="round"/>
            {/* Normal-sized centered submit button */}
            <div class="w-full max-w-md p-6">
                <button type="submit"
                    class="w-full px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-medium
                   rounded-md shadow-sm text-lg focus:outline-none focus:ring-2 focus:ring-offset-2 
                   focus:ring-green-500 transition-colors"
                >
                    Generate Seed
                </button>
            </div>
        </Form>
    }
}
