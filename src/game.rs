use yew::prelude::*;
use std::ops::Deref;
use std::collections::HashMap;
use gloo::console::log;

use crate::agents::agent::Agent;
use crate::components::utils::{cell_type_to_string, generate_map, summon_agent};
use crate::components::map::Map;
use crate::components::cell::CellData;
use crate::components::info::Info;



#[derive(Default, Clone)]
struct GameState {
  pub selected_cell_idx: usize,
  pub selected_cell_type: String,
  pub selected_agent_id: Option<String>,
  pub selected_agent_troops: Option<usize>,
  pub selected_agent_money: Option<usize>,
  pub cells_data: Vec<CellData>,
  pub show_map: bool,
  pub agents: HashMap<String, Agent>,
}


#[function_component(Game)]
pub fn game() -> Html {
  // Initialize game state
  let cells_data = generate_map();
  let game_state = use_state(|| GameState{ selected_cell_idx: 0, selected_cell_type: {"".to_string()}, cells_data: {cells_data}, show_map: true, ..Default::default() });


  // Callbacks
  let cloned_state = game_state.clone();
  let change_cell = Callback::from(move |cell_idx: usize| {
    let mut data = cloned_state.deref().clone();
    data.selected_cell_idx = cell_idx;
    let cell_type = data.cells_data[cell_idx-1].cell_type;
    data.selected_cell_type = cell_type_to_string(cell_type);

    if data.cells_data[cell_idx-1].agent_id.is_some() {
      data.selected_agent_id = data.cells_data[cell_idx-1].agent_id.clone();
      data.selected_agent_troops = Some(data.agents[&data.cells_data[cell_idx-1].agent_id.clone().unwrap()].troops);
      data.selected_agent_money = Some(data.agents[&data.cells_data[cell_idx-1].agent_id.clone().unwrap()].money);
    } else {
      data.selected_agent_id = None;
      data.selected_agent_troops = None;
      data.selected_agent_money = None;
    }

    cloned_state.set(data);
  });

  let cloned_state = game_state.clone();
  let recruit_button_clicked = Callback::from(move |_| {
    if cloned_state.selected_agent_id.is_some() {
      // TODO
      //cloned_state.selected_agent.unwrap().recruit_troops(1);
      log!("Recruitred"); 

      //log!(cloned_state.selected_agent.unwrap().troops);
    } else {
      log!("Select an Agent");
    };
    
  });

  let cloned_state = game_state.clone();
  let generate_new_map_clicked = Callback::from(move |_| {
    log!("New map generated");
    let mut data = cloned_state.deref().clone();
    data.cells_data = generate_map();
    cloned_state.set(data);
  });

  let cloned_state = game_state.clone();
  let generate_new_agent = Callback::from(move |_| {
    log!("New hero summoned");
    let mut data = cloned_state.deref().clone();
    let agent_cell = 2;
    let agent_id = "Hero 1".to_string();
    let new_agent = summon_agent(agent_id.clone(), 1, 1, agent_cell);
    data.agents.insert(agent_id.clone(), new_agent);

    data.cells_data[agent_cell-1].agent_id = Some(agent_id.clone());
    

    cloned_state.set(data);
  });
  
  html! {
    <div class="row g-2">
      <div class="col-9">
        <button onclick={generate_new_map_clicked} type="button" class="btn btn-primary">
          {"Generate New Map"}
        </button>
        <button onclick={generate_new_agent} type="button" class="btn btn-primary">
          {"Summon Hero"}
        </button>
        <Map 
          cells_data={game_state.cells_data.clone()}
          country_name="None" 
          handle_change_cell={change_cell} 
          recruit_troops={recruit_button_clicked}
          show_map={game_state.show_map}
        />
      </div>
      <div class="col-3">
        <Info 
          selected_cell={game_state.selected_cell_idx}
          selected_cell_type={game_state.selected_cell_type.clone()} 
          selected_agent_id={game_state.selected_agent_id.clone()}
          selected_agent_troops={game_state.selected_agent_troops.clone()}
          selected_agent_money={game_state.selected_agent_money.clone()}
        />
      </div>
    </div>
  }
}