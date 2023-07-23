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

