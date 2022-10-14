use rand::Rng;

use crate::components::cell::{CellTypes, CellData};
use crate::components::map::NUM_CELLS;

use crate::agents::agent::Agent;

pub fn generate_map() -> Vec<CellData> {
  let cells_data: Vec<CellData> = (0..NUM_CELLS).map(|idx_cell| {
    let mut rng = rand::thread_rng();
    let rand_int: usize = rng.gen_range(0, 99);
    let cell_type = match rand_int {
      0..=80 => { CellTypes::Grass }
      81..=97 => { CellTypes::Rock }
      _ => { CellTypes::City }
    };
    
    CellData { cell_type: (cell_type), idx_cell: (idx_cell), agent_id: (None) }
  }).collect();

  cells_data
}

pub fn summon_agent(agent_id: String, troops: usize, money: usize, cell_idx: usize) -> Agent {
  Agent::new( agent_id,  troops,  money, cell_idx)
}

pub fn cell_type_to_string(cell_type: CellTypes) -> String {
  let cell_type_string = match cell_type {
    CellTypes::Grass => { "Grass Field".to_string() }
    CellTypes::Rock => { "Rocky Mountains".to_string() }
    CellTypes::City => { "City".to_string() }
  };
  cell_type_string
}