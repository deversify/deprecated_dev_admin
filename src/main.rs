mod components;
mod pages;

use yew::prelude::*;

use pages::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <SignInPage />
    }
}

fn main() {
    yew::start_app::<App>();
}
