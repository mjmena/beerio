use leptos::prelude::*;
use leptos_icons::Icon;

use crate::pages::random_page::generate_random_string;

#[component]
pub fn SplashPage() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-around items-center w-screen bg-red-200 h-svh animate-float-grid bg-[url('data:image/svg+xml,%3Csvg%20width=%27120%27%20height=%27120%27%20viewBox=%270%200%20120%20120%27%20xmlns=%27http://www.w3.org/2000/svg%27%3E%3Cpath%20d=%27M9%200h2v20H9V0zm25.134.84l1.732%201-10%2017.32-1.732-1%2010-17.32zm-20%2020l1.732%201-10%2017.32-1.732-1%2010-17.32zM58.16%204.134l1%201.732-17.32%2010-1-1.732%2017.32-10zm-40%2040l1%201.732-17.32%2010-1-1.732%2017.32-10zM80%209v2H60V9h20zM20%2069v2H0v-2h20zm79.32-55l-1%201.732-17.32-10L82%204l17.32%2010zm-80%2080l-1%201.732-17.32-10L2%2084l17.32%2010zm96.546-75.84l-1.732%201-10-17.32%201.732-1%2010%2017.32zm-100%20100l-1.732%201-10-17.32%201.732-1%2010%2017.32zM38.16%2024.134l1%201.732-17.32%2010-1-1.732%2017.32-10zM60%2029v2H40v-2h20zm19.32%205l-1%201.732-17.32-10L62%2024l17.32%2010zm16.546%204.16l-1.732%201-10-17.32%201.732-1%2010%2017.32zM111%2040h-2V20h2v20zm3.134.84l1.732%201-10%2017.32-1.732-1%2010-17.32zM40%2049v2H20v-2h20zm19.32%205l-1%201.732-17.32-10L42%2044l17.32%2010zm16.546%204.16l-1.732%201-10-17.32%201.732-1%2010%2017.32zM91%2060h-2V40h2v20zm3.134.84l1.732%201-10%2017.32-1.732-1%2010-17.32zm24.026%203.294l1%201.732-17.32%2010-1-1.732%2017.32-10zM39.32%2074l-1%201.732-17.32-10L22%2064l17.32%2010zm16.546%204.16l-1.732%201-10-17.32%201.732-1%2010%2017.32zM71%2080h-2V60h2v20zm3.134.84l1.732%201-10%2017.32-1.732-1%2010-17.32zm24.026%203.294l1%201.732-17.32%2010-1-1.732%2017.32-10zM120%2089v2h-20v-2h20zm-84.134%209.16l-1.732%201-10-17.32%201.732-1%2010%2017.32zM51%20100h-2V80h2v20zm3.134.84l1.732%201-10%2017.32-1.732-1%2010-17.32zm24.026%203.294l1%201.732-17.32%2010-1-1.732%2017.32-10zM100%20109v2H80v-2h20zm19.32%205l-1%201.732-17.32-10%201-1.732%2017.32%2010zM31%20120h-2v-20h2v20z%27%20fill=%27%23ffffff%27%20fill-opacity=%270.4%27%20fill-rule=%27evenodd%27/%3E%3C/svg%3E')]">
        <a
          href=format!("/beerio/solo/?seed={}", generate_random_string())
          class="flex flex-col justify-center items-center"
        >
          <Icon icon=icondata::IoPerson attr:class="size-40 fill-red-500" />
          <h2 class="text-5xl font-bold text-center text-red-500">SOLO MISSIONS</h2>
        </a>
        <a href=format!("/beerio/coop/?seed={}", generate_random_string()) class="flex flex-col">
          <div class="flex justify-center items-center">
            <Icon icon=icondata::IoPerson attr:class="size-40 fill-red-500" />
            <Icon icon=icondata::IoPerson attr:class="size-40 fill-green-500" />
          </div>
          <h2 class="text-5xl font-bold text-center text-green-500">CO-OP MISSIONS</h2>
        </a>
      </div>
    }
}
