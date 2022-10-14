use yew::prelude::*;

use crate::agents::agent::Agent;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CellData {
    pub cell_type: CellTypes,
    pub idx_cell: usize,
    pub agent_id: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellTypes {
  Grass,
  Rock,
  City,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CellProps {
    pub cell_type: CellTypes,
    pub idx_cell: usize,
    pub show_map: bool,
    pub msg_click_cell: Callback<usize>,
    pub is_selected: bool,
    pub agent_id: Option<String>,
}

// Cell component
#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let idx_cell = props.idx_cell;
    let msg_click_cell = props.msg_click_cell.clone();
    let cell_onclick = Callback::from(move |_event: MouseEvent| {
        msg_click_cell.emit(idx_cell);
    });

    let cell_type = match props.cell_type {
        CellTypes::Grass => { "cell-grass" }
        CellTypes::Rock => { "cell-rock" }
        CellTypes::City => { "cell-city" }
    };

    let agent_style = match props.agent_id { 
        Some(_) => { "has-troops" }
        _ => { "no-troops" }
    };


    let cell_html: Html = if !props.show_map {  // Map not shown
        let r: Html = html! {
            <div key={props.idx_cell} class={classes!("empty-cells")}></div>
        };
        r
    } else if props.is_selected {  // Selected cell
        let r: Html = html! {
            <div 
                key={props.idx_cell} 
                class={classes!("selected-cell", "game-cells", cell_type)}
                onclick={cell_onclick}
            >
                <div class={classes!(agent_style)}></div>
            </div>
        };
        r
    } else {  // Standard cell
        let r: Html = html! {
            <div key={props.idx_cell} class={classes!("game-cells", cell_type)}
                onclick={cell_onclick}>
                <p class={classes!(agent_style)}></p>
            </div>
        };
        r
    };
       
    html! {
        {cell_html}
    }
}


