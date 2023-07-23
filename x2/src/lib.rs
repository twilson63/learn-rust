use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp_contracts::{
    handler_result::WriteResult,
    js_imports::{SmartWeave, Transaction},
    warp_contract,
};

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum PstError {
    TransferAmountMustBeHigherThanZero,
    IDontLikeThisContract,
    CallerBalanceNotEnough(u64),
    OnlyOwnerCanEvolve,
    EvolveNotAllowed,
    WalletHasNoBalanceDefined(String),
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    ticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    balances: HashMap<String, u64>,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub qty: u64,
    pub target: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Action {
    //    Balance(Balance),
    Transfer(Transfer),
}

pub trait WriteActionable {
    fn action(self, caller: String, state: State) -> WriteResult<State, PstError>;
}

impl WriteActionable for Transfer {
    fn action(self, _caller: String, mut state: State) -> WriteResult<State, PstError> {
        // if self.qty == 0 {
        //     return WriteResult::ContractError(TransferAmountMustBeHigherThanZero);
        // }
        let caller = Transaction::owner();
        let balances = &mut state.balances;

        let caller_balance = *balances.get(&caller).unwrap_or(&0);
        // if caller_balance < self.qty {
        //     return WriteResult::ContractError(CallerBalanceNotEnough(caller_balance));
        // }

        balances.insert(caller, caller_balance - self.qty);

        let target_balance = *balances.get(&self.target).unwrap_or(&0);
        balances.insert(self.target, target_balance + self.qty);

        WriteResult::Success(state)
    }
}

#[warp_contract(write)]
pub fn handle(state: State, action: Action) -> WriteResult<State, PstError> {
    let caller = SmartWeave::caller();

    match action {
        Action::Transfer(action) => action.action(caller, state),
    }
}
