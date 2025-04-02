use leptos::prelude::*;

#[component]
pub fn Rule(#[prop(into)] rule: Signal<u32>) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100 flex flex-col items-center">
            <div class="w-full max-w-md p-6">
                {rule}

            </div>

            <div class="w-full max-w-md p-6">
                <button
                    class="w-full px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-medium
                   rounded-md shadow-sm text-lg focus:outline-none focus:ring-2 focus:ring-offset-2 
                   focus:ring-green-500 transition-colors"
                >
                    BACK
                </button>
            </div>
        </div>
    }
}
