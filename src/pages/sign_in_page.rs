use crate::components::*;
use yew::prelude::*;

#[function_component(SignInPage)]
pub fn fc() -> Html {
    html! {

        <Container>
            <h1>{"Sign in"}</h1>
            <EmailField/>
            <SignInButton/>
        </Container>
    }
}
