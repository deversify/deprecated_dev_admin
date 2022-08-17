use yew::prelude::*;

#[function_component(SignInButton)]
pub fn sign_in_button() -> Html {
    html! {
        <button class="mdc-button mdc-button--raised">  <span class="mdc-button__ripple"></span>{"Sign in"}</button>
    }
}
