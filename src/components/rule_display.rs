use crate::RULES;
use leptos::prelude::*;

#[component]
pub fn RuleDisplay(#[prop(into)] rule: Signal<u32>) -> impl IntoView {
    let rule = Signal::derive(move || RULES.get(rule.get() as usize).unwrap().clone());

    view! {
        <div class="w-full max-w-md p-6">
            {move || rule.get().name} <br/>
            {move || rule.get().description}
        </div>
    }
}
