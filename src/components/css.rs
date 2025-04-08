use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button
            class="w-full h-full px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-medium
           rounded-md shadow-sm text-lg focus:outline-none focus:ring-2 focus:ring-offset-2 
           focus:ring-green-500 transition-colors"
        >
            {children()}
        </button>
    }
}
