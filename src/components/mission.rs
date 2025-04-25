use crate::{
    Mission,
    components::item_toggle::ItemToggleDisplay,
    pages::random_page::{
        GACHA_ITEMS, ITEMS, RandomCoopSingleMissionDisplay, RandomItemDisplay,
        RandomLoadoutDisplay, RandomNumberDisplay,
    },
};
use leptos::prelude::*;

#[component]
pub fn MissionView(
    get_mission: Signal<Mission>,
    #[prop(into)] get_seed: Signal<[u8; 32]>,
) -> impl IntoView {
    view! {
      <div class="flex flex-col items-center p-4 w-full max-w-lg text-center">
        <div class="w-full text-2xl font-bold">{move || get_mission().name}</div>
        <div class="p-2 w-full text-lg">{move || get_mission().description}</div>
        <Show when=move || !get_mission().details.is_empty()>
          <div class="p-2 text-left">
            <h3 class="text-lg font-semibold">Details</h3>
            <ul class="px-4 list-disc">
              {move || {
                get_mission()
                  .details
                  .into_iter()
                  .map(|detail| view! { <li>{detail}</li> })
                  .collect_view()
              }}
            </ul>
          </div>
        </Show>
        <Show when=move || get_mission().needs_random_item>
          <RandomItemDisplay get_seed />
        </Show>
        <Show when=move || get_mission().needs_random_loadout>
          <RandomLoadoutDisplay get_seed />
        </Show>
        <Show when=move || { get_mission().needs_random_number > 0 }>
          <RandomNumberDisplay get_seed number=get_mission().needs_random_number />
        </Show>
        <Show when=move || !get_mission.get().two_hundred_cc_adjustment.is_empty()>
          <div class="p-2 text-left">
            <h3 class="text-lg font-semibold">200cc Adjustments</h3>
            <ul class="px-4 list-disc">
              <li>{move || get_mission.get().two_hundred_cc_adjustment}</li>
            </ul>
          </div>
        </Show>
        <Show when=move || get_mission.get().needs_item_checklist>
          <div class="p-2 text-left">
            <ItemToggleDisplay items=ITEMS.into() />
          </div>
        </Show>
        <Show when=move || get_mission.get().needs_gacha_item_checklist>
          <div class="p-2 text-left">
            <ItemToggleDisplay items=GACHA_ITEMS.into() />
          </div>
        </Show>
        <Show when=move || get_mission.get().needs_coop_singles>
          <RandomCoopSingleMissionDisplay get_seed />
        </Show>
      </div>
    }
    .into_any()
}
