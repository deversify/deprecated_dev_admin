use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Container)]
pub fn fc(p: &Props) -> Html {
    html! {
        <div style="
            display: flex;
            align-items: center;
            justify-content: center;
            flex-direction: column;
            height: 50vh
            " >
            { for p.children.iter() }
        </div>
    }
}
