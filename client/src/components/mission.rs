use crate::Mission;
use leptos::prelude::*;

#[component]
pub fn MissionView(#[prop(into)] mission: Signal<Mission>) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full max-w-md text-center p-4">
            <div class="w-full text-2xl font-bold">
            {move || mission.get().name}
            </div>
            <div class="w-full">
            {move || mission.get().description}
            </div>
        </div>
    }
}
