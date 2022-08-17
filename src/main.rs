mod email_field;
mod sign_in_button;

use email_field::*;
use sign_in_button::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {

        <section style="
        display: flex;
        align-items: center;
        justify-content: center;
        flex-direction: column;
        height: 50vh
        ">

        <h1>{"Dev Admin"}</h1>

        <EmailField/>
        <SignInButton/>
    </section>
    }
}

fn main() {
    yew::start_app::<App>();
}
