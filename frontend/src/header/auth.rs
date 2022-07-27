use std::collections::HashSet;

use yew::prelude::*;
use yew_agent::{Agent, AgentLink, Dispatched, Dispatcher, HandlerId};

use self::event_bus::DeviceFlowEventBus;

pub mod event_bus;

/// Stores authentication state
pub struct AuthComponent;

impl Component for AuthComponent {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <DeviceFlowController />
        }
    }
}

pub struct DeviceFlowController {
    event_bus: Dispatcher<DeviceFlowEventBus>,
    state: LoginState,
}

pub enum LoginState {
    None,
    Started,
    Canceled,
    Done,
}

impl Component for DeviceFlowController {
    type Message = LoginState;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            event_bus: DeviceFlowEventBus::dispatcher(),
            state: LoginState::None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginState::Started => {
                self.event_bus.send(msg);
                false
            }
            _ => todo!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            LoginState::None => {
                html! {
                    <button onclick={ctx.link().callback(|_| LoginState::Started)}>{"Login"}</button>
                }
            }
            LoginState::Started => {
                html! {
                    <button>{"Cancel"}</button>
                }
            }
            LoginState::Canceled => {
                html! {
                    <button class="disabled">{"Canceled"}</button>
                }
            }
            LoginState::Done => {
                html! {
                    <p>{"Display session store"}</p>
                }
            }
        }
    }
}
