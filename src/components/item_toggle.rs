use leptos::prelude::*;

use crate::pages::random_page::ITEMS;

#[component]
pub fn ItemToggleDisplay<'a>(items: Vec<&'a str>) -> impl IntoView {
    view! {
      <div class="flex flex-wrap flex-grow justify-evenly items-center">
        {items
          .into_iter()
          .map(|item| view! { <ItemToggle item=item.to_string() /> })
          .collect_view()}
      </div>
    }
}

#[component]
pub fn ItemToggle(item: String) -> impl IntoView {
    let (get_is_active, set_is_active) = signal(true);
    view! {
      <img
        class="object-contains"
        class=(["size-15", "md:size-25"], move || get_is_active())
        class=(["size-10", "md:size-20", "order-last", "grayscale"], move || !get_is_active())
        src=format!("assets/items/{}.png", item)
        on:click=move |_| set_is_active(!get_is_active())
      />
    }
}
