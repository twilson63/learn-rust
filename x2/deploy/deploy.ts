import { WarpFactory } from 'warp-contracts';
import { ArweaveSigner, DeployPlugin } from 'warp-contracts-plugin-deploy';
import fs from 'fs';
import path from 'path';

const state = {
  name: 'Foo',
  ticker: 'Bar',
  balances: {
    'vh-NTHVvlKZqRxc8LyyTNok65yQ55a_PJ1zWLb9G2JI': 100
  }
}

async function main() {
  const warp = WarpFactory.forMainnet().use(new DeployPlugin());
  const wallet = JSON.parse(fs.readFileSync(path.join(__dirname, '../../jwk.json'), 'utf-8'));
  const { contractTxId } = await warp.deploy({
    src: fs.readFileSync(path.join(__dirname, '../pkg/rust-contract_bg.wasm')),
    wallet: new ArweaveSigner(wallet),
    initState: JSON.stringify(state),
    wasmGlueCode: path.join(__dirname, '../pkg/rust-contract.js'),
    wasmSrcCodeDir: path.join(__dirname, '../src'),
  });

  console.log(contractTxId);
}

main().catch((e) => console.log(e));