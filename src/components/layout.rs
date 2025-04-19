use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_query_map};

use crate::pages::random_page::generate_random_string;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
      <div class="w-screen h-screen min-h-screen bg-gray-100">
        <div class="flex flex-col items-center pb-16 w-full h-full bg-transparent">
          {children()}
        </div>
      </div>
      <Navbar />
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let (get_next_seed, set_next_seed) = signal(generate_random_string());
    let get_seed = || use_query_map().read().get("seed").unwrap_or_default();

    Effect::new(move || {
        if get_seed() == get_next_seed() {
            set_next_seed(generate_random_string());
        }
    });

    view! {
      <nav class="flex fixed bottom-0 justify-around items-center w-full from-gray-100 to-red-100 bg-linear-to-b">
        <A href="/beerio/" attr:class="flex flex-col justify-center items-center">
          <Icon icon=icondata::IoHome attr:class="size-15 fill-red-500" />
        </A>
        <A
          href=move || format!("./?seed={}", get_next_seed())
          attr:class="flex flex-col justify-center items-center"
        >
          <Icon icon=icondata::IoDice attr:class="size-30 fill-red-500" />
        </A>
        <A href="/beerio/" attr:class="flex flex-col justify-center items-center">
          <Icon icon=icondata::BsSignStop attr:class="size-15 fill-red-500" />
        </A>
      </nav>
    }
}
