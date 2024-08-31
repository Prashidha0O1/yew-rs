use yew::prelude::*;

#[function_component]
fn App() -> Html {
        let count = use_state(|| 0);
        let onclick = {
            let counter = count.clone();
            move |_| {
                let value = *counter + 1;
                counter.set(value);
            }
        };
    html!{
        <title> {"Counter Using Yew"} </title>
        <div>
            <button {onclick}> {"Add"}</button>
            <p>{ *count}</p>
        </div>
    }
}
    

fn main() {
    yew::Renderer::<App>::new().render();
} 
