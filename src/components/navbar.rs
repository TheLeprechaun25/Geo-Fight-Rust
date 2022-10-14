use yew::prelude::*;
use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub logged_in: bool,
  pub handle_login: Callback<()>,
  pub handle_logout: Callback<()>,
}

// Map component
#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
  let logged_in = props.logged_in.clone();
  let handle_logout = props.handle_logout.clone();
  let handle_login = props.handle_login.clone();
  let login = Callback::from(move |_| {
    if logged_in {
      log!("Loging out");
      handle_logout.emit(());
    } else {
      log!("Loging in");
      handle_login.emit(());
    }
  });

  html! {
    <nav class="navbar navbar-expand-lg bg-light">
      <div class="container-fluid">
      
        <a class="navbar-brand" href="#">
          <img src="assets/earth.jpg" alt="" width="24" height="24" class="d-inline-block align-text-top"/>
          {" GEOFIGHT.rs"}
        </a>
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
          <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarSupportedContent">
          <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <li class="nav-item">
              <a class="nav-link active" aria-current="page" href="/">{"Home"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link active" href="/ranking">{"Ranking"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link active" href="/about">{"About"}</a>
            </li>
            <li class="nav-item dropdown">
              <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                {"Links"}
              </a>
              <ul class="dropdown-menu">
                <li><a class="dropdown-item" href="https://geofightnft.com/">{"Website"}</a></li>
                <li><a class="dropdown-item" href="https://twitter.com/GeoFightNFT">{"Twitter"}</a></li>
                <li><a class="dropdown-item" href="mailto:geofight@protonmail.com">{"Mail"}</a></li>
                <li><a class="dropdown-item" href="https://discord.gg/UdtWKbg6Qj">{"Discord"}</a></li>
              </ul>
            </li>
          </ul>
          <button class="btn btn-outline-dark" type="submit" onclick={login}>{
            if props.logged_in {
              "Logout" 
            } else {
              "Login"
            } 
          }</button>
        </div>
      </div>
    </nav>
  }
}