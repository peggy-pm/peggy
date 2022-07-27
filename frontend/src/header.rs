use yew::prelude::*;

pub mod auth;

use auth::AuthComponent;

pub struct NavbarComponent;

impl Component for NavbarComponent {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <nav>
                <AuthComponent />
            </nav>
        }
    }
}
