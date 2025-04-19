use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

use crate::{
    SOLO_MISSIONS,
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
            .unwrap_or(1)
    };
    let mission = Signal::derive(move || SOLO_MISSIONS.get(mission_id() - 1).unwrap().clone());

    view! {
      <div class="flex relative flex-col h-full">
        <MissionView mission />
        <div class="flex absolute bottom-0 gap-x-4 justify-center w-full max-w-md">
          <Show when=move || { mission_id() > 1 }>
            <div class="flex-grow w-1/2">
              <A href=move || format!("../{}", mission_id() - 1)>
                <Button>
                  <div>PREVIOUS</div>
                  <div>MISSION</div>
                </Button>
              </A>
            </div>
          </Show>
          <Show when=move || { mission_id() < SOLO_MISSIONS.len() }>
            <div class="flex-grow w-1/2">
              <A href=move || format!("../{}", mission_id() + 1)>
                <Button>
                  <div>NEXT</div>
                  <div>MISSION</div>
                </Button>
              </A>
            </div>
          </Show>
        </div>
      </div>
    }
}

#[component]
pub fn MissionListPage() -> impl IntoView {
    let missions_list_view = (0..SOLO_MISSIONS.len())
        .map(|i| {
            let mission = SOLO_MISSIONS.get(i).unwrap().clone();
            view! {
              <a
                href=format!("missions/{}", i + 1)
                class="flex flex-col p-6 bg-white rounded-lg border border-gray-200 shadow-sm dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                  {mission.name}
                </h5>
                <p class="font-normal text-gray-700 dark:text-gray-400">{mission.description}</p>
              </a>
            }
        })
        .collect_view();

    view! {
      <h1 class="mb-4 text-6xl font-bold text-center text-gray-800">Missions</h1>
      <div class="flex flex-col flex-grow gap-1 pb-20 w-9/10">{missions_list_view}</div>
    }
}
