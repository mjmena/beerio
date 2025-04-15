use leptos::prelude::*;
use leptos_router::{components::Form, hooks::query_signal};

#[component]
pub fn SeedForm() -> impl IntoView {
    let (get_seed, set_seed) = query_signal::<String>("seed");
    let (get_player, set_player) = query_signal::<usize>("player");

    view! {
      <Form
        method="GET"
        action=move || {
          format!("{}/{}", get_seed().unwrap_or_default(), get_player().unwrap_or(1))
        }
        attr:class="flex flex-col flex-grow items-center w-full"
      >
        <div class="p-6 w-full max-w-md">
          <h1 class="mb-4 text-2xl font-bold text-center text-gray-800">Enter Your Seed</h1>

          {}
          <div class="mb-6">
            <input
              type="text"
              class="py-2 px-4 w-full text-center rounded-md border border-gray-300 shadow-sm focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
              placeholder="Enter game seed"
              prop:value=move || get_seed().unwrap_or_default()
              on:input=move |ev| {
                let value = event_target_value(&ev);
                set_seed(if value.is_empty() { None } else { Some(value) });
              }
            />
          </div>
          <h1 class="mb-4 text-2xl font-bold text-center text-gray-800">Choose Your Player</h1>
        </div>

        {}
        <div class="grid flex-grow grid-cols-3 auto-rows-fr gap-3 p-4 w-full max-w-4xl">
          {(1..=12)
            .map(|player_num| {
              let is_selected = move || get_player().unwrap_or_default() == player_num;
              view! {
                <div class="relative h-full">
                  <input
                    type="radio"
                    id=format!("player_{}", player_num)
                    value=player_num
                    class="absolute w-0 h-0 opacity-0"
                    on:input=move |ev| {
                      let value = event_target_value(&ev);
                      set_player(
                        if value.is_empty() { None } else { Some(value.parse().unwrap_or(1)) },
                      );
                    }
                  />
                  <label
                    for=format!("player_{}", player_num)
                    class=move || {
                      format!(
                        "flex items-center justify-center h-full w-full text-4xl font-bold rounded-xl
                                    border-2 cursor-pointer transition-all duration-200 {}",
                        if is_selected() {
                          "bg-blue-600 text-white border-blue-700 shadow-lg transform scale-[1.03]"
                        } else {
                          "bg-white text-gray-800 border-gray-300 hover:bg-gray-50"
                        },
                      )
                    }
                  >
                    {player_num}
                  </label>
                </div>
              }
            })
            .collect::<Vec<_>>()}
        </div>

        <input type="hidden" value="1" name="round" />
        {}
        <div class="p-6 w-full max-w-md">
          <button
            type="submit"
            class="py-3 px-6 w-full text-lg font-medium text-white bg-green-500 rounded-md shadow-sm transition-colors hover:bg-green-600 focus:ring-2 focus:ring-green-500 focus:ring-offset-2 focus:outline-none"
          >
            Generate Seed
          </button>
        </div>
      </Form>
    }
}
