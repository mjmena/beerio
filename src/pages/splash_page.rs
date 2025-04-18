use leptos::prelude::*;
use leptos_icons::Icon;

use crate::pages::random_page::generate_random_string;

#[component]
pub fn SplashPage() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-around items-center w-screen h-svh">
        <a
          href=format!("/beerio/solo?seed={}", generate_random_string())
          class="flex flex-col items-center"
        >
          <Icon icon=icondata::IoPerson attr:class="size-40 fill-red-500" />
          <h2 class="text-5xl font-bold text-center text-red-500">SOLO MISSIONS</h2>
        </a>
        <a href="./beerio/test" class="flex flex-col">
          <div class="flex justify-center items-center">
            <Icon icon=icondata::IoPerson attr:class="size-40 fill-red-500" />
            <Icon icon=icondata::IoPerson attr:class="size-40 fill-green-500" />
          </div>
          <h2 class="text-5xl font-bold text-center text-green-500">CO-OP MISSIONS</h2>
        </a>
      </div>
    }
}
