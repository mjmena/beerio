use crate::Mission;
use leptos::prelude::*;

#[component]
pub fn MissionView(#[prop(into)] mission: Signal<Mission>) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-1/2 max-w-md text-center text-lg p-4">
            <div class="w-full">
            {move || mission.get().name}
            </div>
            <div class="w-full">
            {move || mission.get().description}
            </div>
        </div>
    }
}
