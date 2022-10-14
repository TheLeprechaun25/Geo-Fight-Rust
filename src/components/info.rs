use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected_cell: usize,
    pub selected_cell_type: String,
    pub selected_agent_id: Option<String>,
    pub selected_agent_troops: Option<usize>,
    pub selected_agent_money: Option<usize>,
}

// Map component
#[function_component(Info)]
pub fn info(props: &Props) -> Html {
    let selected_cell = props.selected_cell.clone();
    let selected_cell_type = props.selected_cell_type.clone();

    let agent_str = match props.selected_agent_id {
        Some(_) => { props.selected_agent_id.clone().unwrap() }
        _ => { "-".to_string() }
    };
    let troops_str = match props.selected_agent_troops {
        Some(_) => { props.selected_agent_troops.unwrap().to_string() }
        _ => { "-".to_string() }
    };
    let money_str = match props.selected_agent_money {
        Some(_) => { props.selected_agent_money.unwrap().to_string() }
        _ => { "-".to_string() }
    };

    html! {
            <div class="p-3 border bg-light">
                <h2> {"Info"} </h2>
                <ul class="list-group">
                    <li class="list-group-item text-start"><p>{"Cell: "}{selected_cell}</p></li>
                    <li class="list-group-item text-start"><p>{"Cell Type: "}{selected_cell_type}</p></li>
                    <li class="list-group-item text-start"><p>{"Agent: "}{agent_str}</p></li>
                    <li class="list-group-item text-start"><p>{"Owner: "}</p></li>
                    <li class="list-group-item text-start"><p>{"Troops: "}{troops_str}</p></li>
                    <li class="list-group-item text-start"><p>{"Treasure: "}{money_str}{"$"}</p></li>
                </ul>
            </div>
    }
}