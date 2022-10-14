use gloo::console::log;

enum Directions {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Agent {
  pub agent_id: String,
  pub troops: usize,
  pub money: usize,
  pub cell_idx: usize,
}

impl Agent {
  pub fn new(agent_id: String, troops: usize, money: usize, cell_idx: usize) -> Self {
    let new_agent = Agent{
      agent_id,
      troops,
      money,
      cell_idx,
    };
    new_agent
  }

  pub fn move_agent(&mut self, new_position: usize) {

    self.cell_idx = new_position;
  }

  fn check_valid_move(self, direction: Directions) -> bool{
    if self.cell_idx == 5 {
      true
    } else {
      false
    }
  }

  pub fn recruit_troops(&mut self, n_troops: usize) {
    self.troops += n_troops;
  }


}