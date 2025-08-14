# TODO

My instructions

run "anvil"

todo. may require manual creation of some file and add some stuff to env…

go to cd contracts.

seems like i have to deploy this first
FOUNDRY_PROFILE=deploy forge script ./lib/sp1-contracts/contracts/script/deploy/SP1VerifierGatewayPlonk.s.sol:SP1VerifierGatewayScript --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80  --broadcast --rpc-url http://127.0.0.1:8545 --chain-id 31337

and then this, which automatically adds the route?
FOUNDRY_PROFILE=deploy forge script ./lib/sp1-contracts/contracts/script/deploy/v4.0.0-rc.3/SP1VerifierPlonk.s.sol:SP1VerifierScript --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80  --broadcast --rpc-url http://127.0.0.1:8545 --chain-id 31337

got this: 0x4e59b44847b379578588920ca78fbf26c0b4956c for sp1 plonk verif.

export VERIFIER=0x4e59b44847b379578588920ca78fbf26c0b4956c

sepolia plonk is this
0x3B6041173B80E77f038f3F2C0f9744f04837185e

export VERIFIER=0xfa6e4cf6d9fbdb0f8788401914679419386afaea

get the vkey.
export PROGRAM_VKEY=0x0040a8f70d0e50dad1c1f2a92684deb37a3bb9c6c796c83b34497b4411fc3fbf

export RPC_URL=http://127.0.0.1:8545

export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

forge create src/VerifyPessimisticProof.sol:VerifyPessimisticProof --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --constructor-args $VERIFIER $PROGRAM_VKEY

got this contract
0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9

debug like this

cast call \
  0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  "getVKey()" \
  --rpc-url $RPC_URL


```
cast send \
  0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  "verifyPessimisticProof(bytes,bytes)" \
  "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003f4b9e2bb63b8a124ca9c44e465dadff6605b3b728a63876df1bc8848fedb709de0a705bad84bfa4ac52ec36a5bd703c313d4a4e0b75985ca3d878c4d6db18a6df86f535e358f771906e88e72d72a6fcfc912181f7f05a690f649ac3920dd395" "0xd4e8ecd20e014c7e4f9cd7a519e21ffb7fd7e1c3e67344c113517ff30b778bad367345d20bc2c10c0215f821090ce1917ca116ec3cfcd587c55410ab0d70693670cbf3e81c69d1aeae55cbb5f43f9102a9e9ad4b7117d3330932f233ea1557ad077bb0f9305ba982aba2078938e3b505f2f0089ffdac695a3bac6f67bcd5a68b65058df9046e6292e592a6294687173dc0b8a305039165bf47b75c78a9ece67e0d352c1a14c5afc1b8c04ef8304e408548310fd156f0f4dd95564990c00a244a9dae88f91c3e5beef698f2eeec6239f11bcf1afe5fc643f17524be33067a2c04e4256d940407e69835095b81093d46e21eb728a7ac1c31d4a75d0764211fa68413026a5c1dc9c9b8b824af00e24abc081c1dbc84ae6d3a4d21c3668396ce958f28f987e01713007fcffd598610b4c497fe8e38a76e7e59277ee31a2edbbb739c28146efa09566f19b8510451ce5758e74761a01ef49887c71315aec027495446be18f48e0d7ebdac831978278e227863bae18f71d2047c912d132b63f83c3064be907fd209c5cea777b1b83170a5a264d0c20f422a5c83b9b4133225cd91d41189f284922fa18b6ab7dbcfc95758118e4e14f3c5024250a5716dc1df2fef23d78688b52a2e0355fdd684a2ddd3338c5730bf16c7ccca2b478d468dfd934fd4e8afa1a4a522f10e5d45df6915bd519f7726bee3a62d2945c3831484f947c64bd3b364aa952ac087fec28b8e70c3deabd9166b5c2a4a3ec100819d668d9d0f3fa47ac7878713bd0da236996c224bec790e852414d3b43602b4eddff61f5813a1b95a08910307fea66809754777374765b9e2f761aba14d5cf16bc23bc299f162e6356da83b1d25f84227b7e433f032148dcf7eb4d4f36663cdec66420c0f34f29ca3e756792a3b755b79e9b5cfa6537e21501f2fd261c01b740b262137e2517011e33f350f1227df676ba4f479080aa8bac194344efbdc1f54046c4efd20981a1b8537b2061979660b9fbcc390bcf2bdcc17932bbc81aefb73e168fe04a26211bc2f7ada2e07e932b5dc19a6e09b9fef53e1532c093a9b58e9f077c61aa82af9d21402cc231776ea8fb6d18c4bb9b9e7637110a8d98d766e5e8457bfa96bc523da011885f6149d1f5ee71fc359ed70b6806901c3a433896aa580f903cc58c251cb30457f302432b1e6fb272473c1f82d6b903e2b892f7bcef7525c3e84e386930a07bcec55" \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY
```

# SP1 Project Template

This is a template for creating an end-to-end [SP1](https://github.com/succinctlabs/sp1) project
that can generate a proof of any RISC-V program.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

## Running the Project

There are 3 main ways to run this project: execute a program, generate a core proof, and
generate an EVM-compatible proof.

### Build the Program

The program is automatically built through `script/build.rs` when the script is built.

### Execute the Program

To run the program without generating a proof:

```sh
cd script
cargo run --release -- --execute
```

This will execute the program and display the output.

### Generate an SP1 Core Proof

To generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default) for your program:

```sh
cd script
cargo run --release -- --prove
```

### Generate an EVM-Compatible Proof

> [!WARNING]
> You will need at least 16GB RAM to generate a Groth16 or PLONK proof. View the [SP1 docs](https://docs.succinct.xyz/docs/sp1/getting-started/hardware-requirements#local-proving) for more information.

Generating a proof that is cheap to verify on the EVM (e.g. Groth16 or PLONK) is more intensive than generating a core proof.

To generate a Groth16 proof:

```sh
cd script
cargo run --release --bin evm -- --system groth16
```

To generate a PLONK proof:

```sh
cargo run --release --bin evm -- --system plonk
```

These commands will also generate fixtures that can be used to test the verification of SP1 proofs
inside Solidity.

### Retrieve the Verification Key

To retrieve your `programVKey` for your on-chain contract, run the following command in `script`:

```sh
cargo run --release --bin vkey
```

## Using the Prover Network

We highly recommend using the [Succinct Prover Network](https://docs.succinct.xyz/docs/network/introduction) for any non-trivial programs or benchmarking purposes. For more information, see the [key setup guide](https://docs.succinct.xyz/docs/network/developers/key-setup) to get started.

To get started, copy the example environment file:

```sh
cp .env.example .env
```

Then, set the `SP1_PROVER` environment variable to `network` and set the `NETWORK_PRIVATE_KEY`
environment variable to your whitelisted private key.

For example, to generate an EVM-compatible proof using the prover network, run the following
command:

```sh
SP1_PROVER=network NETWORK_PRIVATE_KEY=... cargo run --release --bin evm
```
