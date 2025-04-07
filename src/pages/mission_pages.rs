use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

use crate::{
    MISSIONS,
    components::{css::Button, mission::MissionView},
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
            .unwrap_or_default()
    };
    let mission = Signal::derive(move || MISSIONS.get(mission_id() - 1).unwrap().clone());

    view! {

        <div class="min-h-screen h-screen bg-gray-100 flex flex-col items-center">
            <h1 class="text-2xl font-bold text-gray-800 text-center mb-4">Missions</h1>
            <MissionView mission/>
            <div class="absolute bottom-20 flex flex-row w-full max-w-md gap-x-4 items-end justify-center">
                <Show when=move || {mission_id() > 1} >
                    <div class="w-1/2">
                    <A href=move || format!("/beerio/missions/{}", mission_id() - 1) >
                        <Button>PREVIOUS MISSION</Button>
                    </A>
                    </div>
                </Show>
                <Show when=move || {mission_id() < MISSIONS.len()} >
                <div class="w-1/2">
                    <A href=move || format!("/beerio/missions/{}", mission_id() + 1) >
                    <Button>NEXT MISSION </Button>
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
            view! {<a href=format!("/beerio/missions/{}",i)>{mission.name}</a>}
        })
        .collect_view();

    view! {
        <div class="min-h-screen h-screen bg-gray-100 flex flex-col items-center">
            {missions_list_view}
        </div>

    }
}
