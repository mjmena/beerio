use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use leptos_router::hooks::use_url;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
      <div class="w-screen h-screen min-h-screen bg-gray-100">
        <div class="flex flex-col items-center pb-16 w-full h-full">
          <Outlet />
        </div>
      </div>
      <Navbar />
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
      <nav class="flex fixed bottom-0 justify-around items-center w-full h-16">
        <NavbarLink href="/missions".to_string()>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="w-6 h-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
            />
          </svg>
          Missions
        </NavbarLink>
        <NavbarLink href="/".to_string()>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="w-6 h-6"
            fill="currentColor"
            viewBox="0 0 16 16"
          >
            <path d="M13 1a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2zM3 0a3 3 0 0 0-3 3v10a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3V3a3 3 0 0 0-3-3z" />
            <path d="M5.5 4a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m8 0a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m0 8a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m-8 0a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m4-4a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0" />
          </svg>
          Randomizers
        </NavbarLink>
        <NavbarLink href="/seed".to_string()>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="w-6 h-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
            />
          </svg>
          Seed
        </NavbarLink>
      </nav>
    }
}

#[component]
pub fn NavbarLink(href: String, children: Children) -> impl IntoView {
    let url = use_url();
    view! { <A href=move || url().path().to_owned() + &href.clone()>{children()}</A> }
}
