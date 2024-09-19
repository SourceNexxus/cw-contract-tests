Key Features:
Instantiation: A beneficiary address is set when the contract is instantiated.
Vesting: The total 1,000,000 USOURCE tokens are locked, and they vest linearly over 360 days.

Claiming: The beneficiary can claim their vested tokens at any point, based on how many days have passed.

How It Works:
The instantiate function sets the beneficiary and the contract's start time.

The calculate_vested_amount function calculates how many tokens are vested based on elapsed time.

The try_claim function allows the beneficiary to claim the vested tokens once the time has passed. It calculates the amount that can be claimed, sends it, and updates the claimed state.

Assumptions:
The contract assumes usource has 6 decimal places (if it's like uosmo, ustake, etc.).
The tokens are released in one-day intervals.

Improvements:
Owner controls for withdrawing the remaining funds or canceling the vesting schedule.

Security features such as pausing or revoking the vesting process.

-----------------------------------------------------------------------------------------


Step 1: Install the Required Tools
Before you start, ensure you have installed the following tools on your development environment:

Rust and Cargo: The contract is written in Rust, so you'll need Rust's package manager, Cargo.

Install Rust via rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Wasm32 Target: You will need to add the wasm32-unknown-unknown target to compile the smart contract to WebAssembly.

```bash
rustup target add wasm32-unknown-unknown
```

Source Blockchain CLI (sourced): Install the sourced CLI tool. Follow the Source Blockchain documentation for installation instructions.

Step 2: Clone and Setup CosmWasm Template
You can start by cloning a CosmWasm contract template, or you can use the contract you have written. If starting from scratch, here's how to use a template:

```bash
git clone https://github.com/CosmWasm/cosmwasm-template.git
cd cosmwasm-template
```

If you're using the contract you created above, replace the contract in the template with your code.

Step 3: Modify the Cargo.toml File
In the Cargo.toml file, ensure you have the correct dependencies for CosmWasm and the version you are using. For example:

Cargo.toml
```bash
[package]
name = "vesting-contract"
version = "0.1.0"
edition = "2018"

[dependencies]
cosmwasm-std = { version = "1.0", default-features = false }
cosmwasm-storage = { version = "1.0" }
cosmwasm-schema = { version = "1.0" }
serde = { version = "1.0", features = ["derive"] }
schemars = "0.8"
bincode = "1.3"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies.cw2]
version = "0.10.0"

[profile.release]
opt-level = "z"
```

Step 4: Compile the Contract
Next, compile the smart contract to a WebAssembly binary:

```bash
cargo wasm
``
This will generate a contract.wasm file in the target/wasm32-unknown-unknown/release/ directory.

To optimize the Wasm binary (recommended for deployment), install and use wasm-opt:

```bash
# Install wasm-opt (if it's not installed)
brew install binaryen  # For macOS
# or
sudo apt-get install binaryen  # For Ubuntu
```

# Optimize Wasm binary
```bash
wasm-opt -Oz target/wasm32-unknown-unknown/release/vesting_contract.wasm -o contract_optimized.wasm
```

Step 5: Setup sourced CLI
Initialize your sourced environment:

```bash
sourced init <your-node-name> --chain-id source-chain-id
```
Add keys (create or import your wallet):

```bash
sourced keys add <wallet-name>
```

Or import an existing wallet using:

```bash
sourced keys import <wallet-name> <path-to-keyfile>
```

Fund your wallet with test tokens (if applicable).

Step 6: Upload the Contract
Now that your contract is compiled, you can upload it to the Source Blockchain.

First, ensure your sourced is configured with the correct node and chain ID:

```bash
sourced config chain-id source-chain-id
sourced config node tcp://localhost:26657
```

Upload the optimized Wasm binary to the blockchain:

```bash
sourced tx wasm store contract_optimized.wasm --from <wallet-name> --gas auto --gas-adjustment 1.5 --fees 1000usource
```

--from <wallet-name>: Specifies the wallet you are using.
--gas auto: Automatically estimates the gas.
--fees 1000usource: The fee for the transaction.
Take note of the code ID returned from this transaction. You will need it for contract instantiation.

Step 7: Instantiate the Contract
After successfully uploading the contract, instantiate it by providing the necessary parameters (e.g., the beneficiary’s address):

```bash
sourced tx wasm instantiate <code-id> '{"beneficiary": "<beneficiary-address>"}' --from <wallet-name> --label "vesting-contract" --gas auto --fees 1000usource
```

Replace <code-id> with the code ID you received in the previous step.
Replace <beneficiary-address> with the beneficiary’s wallet address.
You should get a transaction hash as confirmation and a contract address, which you will use to interact with the contract.

Step 8: Interact with the Contract
Once the contract is deployed, you can interact with it using the sourced CLI.

Claiming Tokens
To claim vested tokens, the beneficiary can send an execute message:

```bash
sourced tx wasm execute <contract-address> '{"claim": {}}' --from <beneficiary-wallet> --gas auto --fees 1000usource
```

Replace <contract-address> with the contract address from the instantiation step.
Withdraw or Cancel (Owner Only)
If the owner (you) wants to withdraw or cancel the vesting contract, you can implement the following function in the contract (as per your request) and then execute it using a similar command to the above, specifying the function.

Pausing or Revoking (Owner Only)
You can similarly implement functions for pausing or revoking the contract and use a similar interaction approach.

Step 9: Query Contract State
You can also query the current state of the contract:

```bash
sourced query wasm contract-state smart <contract-address> '{"state": {}}'
This will return the current vesting state, such as the amount claimed so far.
```

Additional Considerations:
Ensure that your node is synced and that you have sufficient testnet tokens for gas fees.
You may want to add contract admin/ownership logic to allow for contract updates or revocations.
