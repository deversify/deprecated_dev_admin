use crate::components::*;
use yew::prelude::*;

pub enum Msg {
    SetInputEnabled(bool),
}

pub struct SignInPage {
    input_enabled: bool,
}

impl Component for SignInPage {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            input_enabled: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInputEnabled(enabled) => {
                if self.input_enabled != enabled {
                    self.input_enabled = enabled;
                    true // Re-render
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {

            <div style="
            display: flex;
            align-items: center;
            justify-content: center;
            flex-direction: column;
            height: 50vh
            " >
                <h1>{"Sign in"}</h1>
                <EmailField/>
                <SignInButton/>
            </div>
        }
    }
}
