use material_yew::text_inputs::MatTextField;
use yew::prelude::*;

#[function_component(EmailField)]
pub fn email_field() -> Html {
    html! {
    <MatTextField label="Email" />
        }
}
