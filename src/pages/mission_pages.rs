use crate::{
    COOP_MISSIONS, Mission, SOLO_MISSIONS,
    components::{layout::Layout, mission::MissionView},
    pages::random_page::{generate_random_string, string_to_sha256},
};
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_params_map, use_query_map};

#[component]
pub fn SoloMissionPage() -> impl IntoView {
    view! { <MissionPage missions=SOLO_MISSIONS.to_vec() /> }
}
#[component]
pub fn CoopMissionPage() -> impl IntoView {
    view! { <MissionPage missions=COOP_MISSIONS.to_vec() /> }
}

#[component]
pub fn MissionPage(missions: Vec<Mission>) -> impl IntoView {
    //collect url /missions/:mission
    let params = use_params_map();
    let mission_id = move || {
        params
            .read()
            .get("id")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1)
    };

    let get_seed = Signal::derive(move || {
        let seed = use_query_map()
            .read()
            .get("seed")
            .unwrap_or(generate_random_string());
        string_to_sha256(&seed)
    });

    let get_mission = Signal::derive(move || missions.get(mission_id() - 1).unwrap().clone());

    view! {
      <Layout>
        <div class="flex relative flex-col h-full">
          <MissionView get_mission get_seed />
        </div>
      </Layout>
    }
}

#[component]
pub fn SoloMissionListPage() -> impl IntoView {
    view! { <MissionListPage missions=SOLO_MISSIONS.to_vec() /> }
}

#[component]
pub fn CoopMissionListPage() -> impl IntoView {
    view! { <MissionListPage missions=COOP_MISSIONS.to_vec() /> }
}

#[component]
pub fn MissionListPage(missions: Vec<Mission>) -> impl IntoView {
    let missions_list_view = (0..missions.len())
        .map(|i| {
            let mission = missions.get(i).unwrap().clone();
            view! {
              <a
                href=move || {
                  format!(
                    "{}{}?seed={}",
                    use_location().pathname.get(),
                    i + 1,
                    generate_random_string(),
                  )
                }
                class="flex flex-col p-6 rounded-lg border border-gray-200"
              >
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900">{mission.name}</h5>
                <p class="font-normal text-gray-700">{mission.description}</p>
              </a>
            }
        })
        .collect_view();

    view! {
      <Layout>
        <h1 class="mb-4 text-6xl font-bold text-center text-gray-800">Missions</h1>
        <div class="flex overflow-scroll flex-col flex-grow gap-1 pb-20 w-9/10">
          {missions_list_view}
        </div>
      </Layout>
    }
}
