use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

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
    view! {
      <nav class="flex fixed bottom-0 justify-around items-center w-full from-gray-100 to-red-100 bg-linear-to-b">
        <NavbarLink href=".".to_string()>
          <Icon icon=icondata::IoDice attr:class="size-30 fill-red-500" />
        </NavbarLink>
      </nav>
    }
}

#[component]
pub fn NavbarLink(href: String, children: Children) -> impl IntoView {
    view! {
      <A href=href.clone() attr:class="flex flex-col justify-center items-center">
        {children()}
      </A>
    }
}
