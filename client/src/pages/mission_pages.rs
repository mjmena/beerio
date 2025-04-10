use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

use crate::{
    components::{css::Button, mission::MissionView},
    MISSIONS,
};

#[component]
pub fn MissionPage() -> impl IntoView {
    //collect url /missions/:mission
    let params = use_params_map();
    let mission_id = move || {
        params
            .read()
            .get("mission")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1)
    };
    let mission = Signal::derive(move || MISSIONS.get(mission_id() - 1).unwrap().clone());

    view! {
    <div class="flex flex-col h-full relative">
        <MissionView mission/>
        <div class="absolute bottom-0 flex w-full max-w-md gap-x-4 justify-center">
            <Show when=move || {mission_id() > 1} >
                <div class="w-1/2 flex-grow">
                <A href=move || format!("/missions/{}", mission_id() - 1) >
                    <Button><div>PREVIOUS</div><div>MISSION</div></Button>
                </A>
                </div>
            </Show>
            <Show when=move || {mission_id() < MISSIONS.len()} >
            <div class="w-1/2 flex-grow">
                <A href=move || format!("/missions/{}", mission_id() + 1) >
                <Button><div>NEXT</div><div>MISSION</div></Button>
            </A>
            </div>
            </Show>
        </div>
    </div>
    }
}

#[component]
pub fn MissionListPage() -> impl IntoView {
    let missions_list_view = (0..MISSIONS.len())
        .map(|i| {
            let mission = MISSIONS.get(i).unwrap().clone();
            view! {
                <a href=format!("/missions/{}",i+1) class="flex flex-col p-6 bg-white border border-gray-200 rounded-lg shadow-sm hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{mission.name}</h5>
                <p class="font-normal text-gray-700 dark:text-gray-400">{mission.description}</p>
            </a>
            }
        })
        .collect_view();

    view! {
        <h1 class="text-6xl font-bold text-gray-800 text-center mb-4">Missions</h1>
        <div class="flex flex-grow flex-col w-9/10 pb-20 gap-1">
                {missions_list_view}
        </div>
    }
}
