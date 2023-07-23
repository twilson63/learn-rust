use serde::{Deserialize, Serialize};
use warp_contracts::{
    handler_result::{ViewResult, WriteResult},
    warp_contract,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    x: u8,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    x: u8,
}

#[derive(Debug, Serialize)]
pub struct View {
    x: u8,
}

#[warp_contract(write)]
pub fn handle(mut state: State, action: Action) -> WriteResult<State, ()> {
    state.x = action.x;
    return WriteResult::Success(state);
}

#[warp_contract(view)]
pub fn view(state: &State, _action: Action) -> ViewResult<View, ()> {
    return ViewResult::Success(View { x: state.x });
}
