A basic CosmWasm smart contract that locks 1,000,000 USOURCE tokens for one year (365 days). After the lock period, the tokens can be withdrawn by the owner. The contract includes features such as:

Locking a specific amount of tokens for one year.
Allowing only the owner of the tokens to withdraw after the lock period.
A simple way to extend or reduce the lock time if needed.

How the Locker Contract Works
Initialization:

The contract is instantiated with an owner address.
The tokens are locked for one year (365 days).
Lock Period:

The lock period is hardcoded to 1 year (365 * 24 * 60 * 60 seconds).
Tokens remain locked until the specified time has passed.
Withdrawal:

Only the owner can withdraw the tokens after the lock period has passed.
After the tokens are withdrawn, the contract marks them as unlocked, and they cannot be withdrawn again.
State Query:

You can query the contract's state to check if the tokens are still locked, who the owner is, and how much time has passed.

Key Features:
Owner-Based Control: Only the specified owner can withdraw the tokens.
Time-Locked: The tokens are locked for exactly one year (365 days).
Security: Once the lock period has passed, the tokens are unlocked and can be withdrawn only once.

------------------------------------------------------------------


Steps to Deploy the Contract
To deploy this contract to the Source Blockchain using the sourced CLI, you can follow the same steps outlined in the previous guide for the vesting contract:

Install the required tools (Rust, CosmWasm, sourced CLI).

Compile the contract to WebAssembly (Wasm).

```bash
cargo wasm
```
Optimize the Wasm binary using wasm-opt:

```bash
wasm-opt -Oz target/wasm32-unknown-unknown/release/locker_contract.wasm -o contract_optimized.wasm
```
Upload the Wasm binary to the blockchain using the sourced CLI:

```bash
sourced tx wasm store contract_optimized.wasm --from <your-wallet> --gas auto --fees 1000usource
```
Instantiate the contract:

```bash
sourced tx wasm instantiate <code-id> '{"owner": "<your-address>"}' --from <your-wallet> --label "locker-contract" --gas auto --fees 1000usource
```
Interact with the contract to withdraw after one year:

```bash
sourced tx wasm execute <contract-address> '{"withdraw": {}}' --from <your-wallet> --gas auto --fees 1000usource
```
