// use yew::prelude::*;

// use super::feed;
// use super::footer;
// use super::side_bar;

// #[function_component(App)]
// pub fn app() -> Html {
//     html! {
//         <main>
//             <div class="flex flex-col h-full">
//                 <div class="flex flow-row flex-wrap ju h-full">
//                     <side_bar::SideBar/>
//                     <feed::Feed/>
//                 </div>
//                 <footer::Footer/>
//             </div>
//         </main>
//     }
// }

use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count(3);
            }
        >
            "Click me: "
            {move || count()}
        </button>
    }
}
