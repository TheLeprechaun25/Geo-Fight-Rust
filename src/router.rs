use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::ranking::Ranking;
use crate::pages::about::About;

use crate::game::Game;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/ranking")]
    Ranking,
    #[at("/about")]
    About,
}

pub fn switch(routes: &Route) -> Html {
  match routes {
      Route::Home => html! { 
        <Game />
      },
      Route::Ranking => html! {
        <Ranking/>
      },
      Route::About => html! { 
        <About/> 
      },
  }
}


