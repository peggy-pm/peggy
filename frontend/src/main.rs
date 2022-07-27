#![feature(type_alias_impl_trait)]

use log::Level;
use yew::prelude::*;

#[macro_use]
extern crate log;

pub mod error;
pub mod header;

pub use error::ErrorPrompt;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <header::NavbarComponent />
        }
    }
}
fn main() -> anyhow::Result<()> {
    console_log::init_with_level(match cfg!(debug_assertions) {
        true => Level::Debug,
        false => Level::Error,
    })?;
    info!("Starting app...");
    yew::start_app::<App>();
    Ok(())
}
