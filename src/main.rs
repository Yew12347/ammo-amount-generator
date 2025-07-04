use yew::prelude::*;
use js_sys::Math;

#[function_component(App)]
fn app() -> Html {
    let random_number = (Math::random() * 100.0).floor() as u32 + 1;

    html! {
        { random_number }
    }
}

fn main() {
    yew::start_app::<App>();
}