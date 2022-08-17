use material_yew::MatButton;
use yew::prelude::*;

#[function_component(SignInButton)]
pub fn sign_in_button() -> Html {
    html! {
    //     <button class="mdc-button mdc-button--raised">  <span class="mdc-button__ripple"></span>  <i class="material-icons mdc-button__icon" aria-hidden="true"
    //     >{"bookmark"}</i
    //   >{"Sign in"}</button>
    <MatButton label="Sign in" raised=true />

    }
}
