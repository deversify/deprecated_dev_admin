mod sign_in_button;

use sign_in_button::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <section style="
        display: flex;
        align-items: center;
        justify-content: center;
        flex-direction: column;">

        <h1>{"Dev Admin"}</h1>


        <SignInButton/>
    </section>
    }
}

fn main() {
    yew::start_app::<App>();
}
