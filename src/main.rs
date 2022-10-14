use yew::prelude::*;
use yew_router::prelude::*;
use std::ops::Deref;

mod router;
mod game;
pub mod components;
pub mod agents;
pub mod pages;

use components::navbar::Navbar;
use crate::router::{switch, Route};

fn main() {
    yew::start_app::<App>();
}

#[derive(Default, Clone)]
pub struct State {
  pub logged_in: bool,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| State{ logged_in: false});

    let cloned_state = state.clone();
    let handle_login = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.logged_in = true;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let handle_logout = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.logged_in = false;
        cloned_state.set(data);
    });

    html! { 
        <>
        <main class="container">
            <div class="container text-center">
                <Navbar logged_in={state.logged_in.clone()} handle_login={handle_login} handle_logout={handle_logout}/>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        </main>
        </>
    }
}