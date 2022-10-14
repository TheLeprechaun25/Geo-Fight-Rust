use yew::prelude::*;
use std::ops::Deref;
use std::collections::HashMap;
use crate::{components::cell::{Cell, CellData}, agents::agent::Agent};

pub const MAP_WIDTH: usize = 50;
pub const MAP_HEIGHT: usize = 50;
pub const NUM_CELLS: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Default, Clone)]
pub struct MapState {
  pub selected_cell_idx: usize,
  pub selected_cell_type: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub cells_data: Vec<CellData>,
  pub country_name: String,
  pub handle_change_cell: Callback<usize>,
  pub recruit_troops: Callback<()>,
  pub show_map: bool,
}


// Map component
#[function_component(Map)]
pub fn map(props: &Props) -> Html {

  let map_state = use_state(|| MapState { selected_cell_idx: 0, selected_cell_type: {"".to_string()}});

  let recruit_troops = props.recruit_troops.clone();
  let recruit_onclick = Callback::from(move |_| {
      recruit_troops.emit(());
  });

  let cloned_state = map_state.clone();
  let handle_change_cell = props.handle_change_cell.clone();
  let cell_clicked = Callback::from(move |cell_idx: usize| {
    let mut data = cloned_state.deref().clone();
    data.selected_cell_idx = cell_idx;
    cloned_state.set(data);
    handle_change_cell.emit(cell_idx);
  });
  
  let cell_rows = (0..MAP_HEIGHT).into_iter().map(|idx_row:usize| {
    html! {
      <div key={idx_row} class="game-row">
        {
          for (0..MAP_WIDTH).into_iter().map(|idx_col| {
            let idx_cell = idx_row * MAP_WIDTH + idx_col + 1;
            html! {
              <Cell
                cell_type={props.cells_data[idx_cell-1].cell_type}
                idx_cell={idx_cell}
                show_map={props.show_map}
                msg_click_cell={cell_clicked.clone()}
                is_selected={idx_cell == map_state.selected_cell_idx}
                agent_id={props.cells_data[idx_cell-1].agent_id.clone()}
              />
            }
          })
        }
      </div>
    }
  });
  html! {
    <>
      <div class="p-3 border bg-light">
        <div class="wrapper">

            <div class="game-field">
              { for cell_rows }
            </div>
        </div>
      </div>
      <button onclick={recruit_onclick} type="button" class="btn btn-primary">
        {"Recruit troops"}
      </button>

    </>
  }
}