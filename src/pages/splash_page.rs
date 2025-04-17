use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn SplashPage() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-around items-center h-full bg-transparent">
        <a
          href="./solo"
          class="flex flex-col items-center p-20 bg-amber-200 rounded-full border-4 border-amber-300"
        >
          <Icon icon=icondata::IoPerson attr:class="size-40 fill-red-500" />
          <h2 class="text-5xl font-bold text-center text-red-500">SOLO MISSIONS</h2>
        </a>
        <a
          href="./coop"
          class="flex flex-col p-16 bg-amber-200 rounded-full border-4 border-amber-300"
        >
          <div class="flex justify-center">
            <Icon icon=icondata::IoPerson attr:class="size-40 fill-red-500" />
            <Icon icon=icondata::IoPerson attr:class="size-40 fill-green-500" />
          </div>
          <h2 class="text-5xl font-bold text-center text-green-500">CO-OP MISSIONS</h2>
        </a>
      </div>
    }
}
