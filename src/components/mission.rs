use crate::{Mission, components::item_toggle::ItemToggleDisplay};
use leptos::prelude::*;

#[component]
pub fn MissionView(#[prop(into)] mission: Signal<Mission>) -> impl IntoView {
    view! {
      <div class="flex flex-col p-4 w-full max-w-lg text-center">
        <div class="w-full text-2xl font-bold">{move || mission.get().name}</div>
        <div class="p-2 w-full text-lg">{move || mission.get().description}</div>
        <Show when=move || !mission.get().details.is_empty()>
          <div class="p-2 text-left">
            <h3 class="text-lg font-semibold">Details</h3>
            <ul class="px-4 list-disc">
              {move || {
                mission
                  .get()
                  .details
                  .into_iter()
                  .map(|detail| view! { <li>{detail}</li> })
                  .collect_view()
              }}
            </ul>
          </div>
        </Show>
        <Show when=move || !mission.get().two_hundred_cc_adjustment.is_empty()>
          <div class="p-2 text-left">
            <h3 class="text-lg font-semibold">200cc Adjustments</h3>
            <ul class="px-4 list-disc">
              <li>{move || mission.get().two_hundred_cc_adjustment}</li>
            </ul>
          </div>
        </Show>
        <Show when=move || mission.get().needs_item_checklist>
          <div class="p-2 text-left">
            <ItemToggleDisplay />
          </div>
        </Show>
      </div>
    }
}
