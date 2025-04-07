use leptos::prelude::*;

#[component]
pub fn Mission(mission: Mission) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-1/2 max-w-md text-center text-lg p-4">
            <div class="w-full">
            {mission.name}
            </div>
            <div class="w-full">
            {mission.description}
            </div>
        </div>
    }
}
