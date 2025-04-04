use crate::RULES;
use leptos::prelude::*;

#[component]
pub fn RuleDisplay(#[prop(into)] rule: Signal<usize>) -> impl IntoView {
    let rule = Signal::derive(move || RULES.get().unwrap().get(rule.get()).unwrap().clone());

    view! {
        <div class="flex flex-col w-full h-1/2 max-w-md text-center text-lg gap-y-10">
            <div class="w-full">
            {move || rule.get().name}
            </div>
            <div class="w-full">
            {move || rule.get().description}
            </div>
        </div>
    }
}
