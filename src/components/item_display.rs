use leptos::prelude::*;

#[component]
pub fn ItemDisplay(#[prop(into)] item_id: Signal<usize>) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-1/2 max-w-md text-center text-4xl p-4">
            <img src=move || format!("/assets/items/{}.png", ITEMS[item_id.get()] ) />
        </div>
    }
}

pub static ITEMS: [&str; 22] = [
    "banana",
    "blooper",
    "bob-omb",
    "boo",
    "boomerang_flower",
    "bullet_bill",
    "coin",
    "crazy_eight",
    "fire_flower",
    "golden_mushroom",
    "green_shell",
    "lightning",
    "mushroom",
    "piranha_plant",
    "red_shell",
    "spiny_shell",
    "super_horn",
    "super_star",
    "triple_banana",
    "triple_green_shells",
    "triple_mushrooms",
    "triple_red_shells",
];
