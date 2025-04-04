use crate::RULES;
use leptos::prelude::*;

#[component]
pub fn RuleDisplay(#[prop(into)] rule: Signal<usize>) -> impl IntoView {
    let rule = Signal::derive(move || RULES.get().unwrap().get(rule.get()).unwrap().clone());

    view! {
        <div class="w-full max-w-md p-6">
            {move || rule.get().name} <br/>
            {move || rule.get().description}
        </div>
    }
}
