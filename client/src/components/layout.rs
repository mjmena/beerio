use leptos::prelude::*;
use leptos_router::components::{Outlet, A};

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="min-h-screen h-screen w-screen bg-gray-100 ">
            <div class="w-full h-full flex flex-col items-center pb-16 ">
                <Outlet />
            </div>
        </div>
        <Navbar/>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
    <nav class="fixed bottom-0 w-full flex justify-around items-center h-16">
            <A href="/missions" attr:class="bg-blue-600 text-white flex flex-col flex-grow items-center justify-center p-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                </svg>
                <span class="text-xs mt-1">Missions</span>
            </A>
            <A href="/" attr:class="bg-blue-600 text-white flex flex-col flex-grow items-center justify-center p-2">
                <svg xmlns="http://www.w3.org/2000/svg" class ="h-6 w-6" fill="currentColor" viewBox="0 0 16 16">
                  <path d="M13 1a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2zM3 0a3 3 0 0 0-3 3v10a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3V3a3 3 0 0 0-3-3z"/>
                  <path d="M5.5 4a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m8 0a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m0 8a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m-8 0a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m4-4a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0"/>
                </svg>
                <span class="text-xs mt-1">Randomizers</span>
            </A>
            <A href="/seed" attr:class="bg-blue-600 text-white flex flex-col flex-grow items-center justify-center p-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
                <span class="text-xs mt-1">Seed</span>
            </A>
        </nav>
    }
}

// #[component]
// pub fn NavbarLink(children: Children) -> impl IntoView {
//     view! {
//         <A href="/missions" attr:class="bg-blue-600 text-white flex flex-col items-center justify-center p-2">
//             <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
//                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
//             </svg>
//             // <span class="text-xs mt-1">{children}</span>
//         </A>
//     }
// }
