use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "PID Viewer" }</h1>
            <p>{ "This is a simple PID viewer." }</p>
            <p>{ "It is a work in progress." }</p>
            // TODO: Add the PID component here
        </main>
    }
}
