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
    }
}

#[component]
pub fn MissionListPage() -> impl IntoView {
    let missions_list_view = (0..MISSIONS.len())
        .map(|i| {
            let mission = MISSIONS.get(i).unwrap().clone();
            view! {<a href=format!("/beerio/missions/{}",i)>
                <div class="text-2xl">{mission.name}</div>
            </a>}
        })
        .collect_view();

    view! {
        <h1 class="text-6xl font-bold text-gray-800 text-center mb-4">Missions</h1>
        {missions_list_view}
    }
}
