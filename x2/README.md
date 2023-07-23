# Smart Weave Contracts with RUST

## Setup

Install NodeJS

https://nodejs.org

Install Rust

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Install Wasmpack

`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

Install Rust Dependencies

```
cargo init --lib
cargo add js-sys@0.3.61
cargo add serde
cargo add serde-wasm-bindgen
cargo add wasm-bindgen@0.2.84
cargo add wasm-bindgen-futures@0.4.34
cargo add warp-contracts
```

Add lib section to Cargo.toml

```
[lib]
crate-type = ["cdylib"]
```

## Setup Nodejs

NodeJS is used to deploy your rust contract

```
yarn init -y
yarn add arweave warp-contracts warp-contracts-plugin-deploy typescript
yarn add -D ts-node @types/node
```

## Add build and deploy scripts

``` json
"scripts": {
  "build": "wasm-pack build --target nodejs --release --out-name rust-contract .",
  "deploy": "yarn build && ts-node deploy/deploy.ts"
}
```

deploy/deploy.ts

```ts
import { WarpFactory } from 'warp-contracts';
import { ArweaveSigner, DeployPlugin } from 'warp-contracts-plugin-deploy';
import fs from 'fs';
import path from 'path';

class State {
  constructor(public x: number) { }
}

async function main() {
  const warp = WarpFactory.forMainnet().use(new DeployPlugin());
  const wallet = JSON.parse(fs.readFileSync(path.join(__dirname, '../../jwk.json'), 'utf-8'));
  const { contractTxId } = await warp.deploy({
    src: fs.readFileSync(path.join(__dirname, '../pkg/rust-contract_bg.wasm')),
    wallet: new ArweaveSigner(wallet),
    initState: JSON.stringify(new State(0)),
    wasmGlueCode: path.join(__dirname, '../pkg/rust-contract.js'),
    wasmSrcCodeDir: path.join(__dirname, '../src'),
  });

  console.log(contractTxId);
}

main().catch((e) => console.log(e));
```

## Build src/lib.rs

```rs
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

```