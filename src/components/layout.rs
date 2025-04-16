use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::{A, Outlet};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
      <div class="w-screen h-screen min-h-screen bg-gray-100">
        <div class="flex flex-col items-center pb-16 w-full h-full">{children()}</div>
      </div>
      <Navbar />
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
      <nav class="flex fixed bottom-0 justify-around items-center w-full h-16">
        <NavbarLink href="missions".to_string()>Missions</NavbarLink>
        <NavbarLink href=".".to_string()>
          <Icon
            icon=icondata::IoDice
            // width="3rem"
            // height="3rem"
            attr:class="transition duration-500 transform fill-red-300 size-30 hover:size-40 hover:fill-red-500"
          />
        </NavbarLink>
        <NavbarLink href="/".to_string()>Seed</NavbarLink>
      </nav>
    }
}

#[component]
pub fn NavbarLink(href: String, children: Children) -> impl IntoView {
    view! {
      <A href=href.clone() attr:class="flex flex-col justify-center items-center p-2">
        {children()}
      </A>
    }
}
