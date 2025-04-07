use leptos::prelude::*;

use crate::{
    RULES,
    components::{layout::Navbar, rule_display::RuleDisplay},
};

#[component]
pub fn RulesDisplay() -> impl IntoView {
    let number_of_rules = RULES.get().unwrap().len();

    let rules_view = (0..number_of_rules)
        .map(|i| {
            view! {<RuleDisplay rule=i/>}
        })
        .collect_view();

    view! {
        <div class="min-h-screen h-screen bg-gray-100 flex flex-col items-center">
            {rules_view}
        </div>
        <Navbar />

    }
}
