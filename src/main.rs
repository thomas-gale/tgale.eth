// mod app;
// mod feed;
// mod footer;
// mod side_bar;

// use app::App;

// fn main() {
//     yew::start_app::<App>();
// }

use leptos::*;

mod app;
use app::App;

fn main() {
    mount_to_body(|| view! { <App/> })
}
