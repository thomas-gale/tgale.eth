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
    let double_count = move || count() * 2;

    view! {
      <div class="flex flex-col h-full">
        <button
            on:click=move |_| {
                set_count.update(|c| *c += 1)
            }
            class="btn btn-primary rounded m-4"
            class=("bg-blue-500", move || double_count() <= 5)
            class=("bg-red-500", move || double_count() > 5)
        >
            "Click me: "
            {count}
            " double: "
            {double_count}
        </button>
      </div>
    }
}
