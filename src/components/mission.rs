use crate::Mission;
use leptos::prelude::*;

use crate::{MISSIONS, components::layout::Navbar};

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

#[component]
pub fn MissionListView() -> impl IntoView {
    let rules_view = (0..MISSIONS.len())
        .map(|i| {
            view! {<MissionView mission=Signal::derive(move || MISSIONS.get(i).unwrap().clone())/>}
        })
        .collect_view();

    view! {
        <div class="min-h-screen h-screen bg-gray-100 flex flex-col items-center">
            {rules_view}
        </div>
        <Navbar />

    }
}
