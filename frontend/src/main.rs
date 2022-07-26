use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class="text-white">{ "Hello world!" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
