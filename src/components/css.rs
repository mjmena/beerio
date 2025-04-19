use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
      <button class="py-3 px-6 w-full h-full text-lg font-medium text-white bg-green-500 rounded-md shadow-sm transition-colors hover:bg-green-600 focus:ring-2 focus:ring-green-500 focus:ring-offset-2 focus:outline-none">
        {children()}
      </button>
    }
}
