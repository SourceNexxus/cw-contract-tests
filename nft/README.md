Steps to Compile and Deploy
Install CosmWasm and Rust: Ensure you have Rust and CosmWasm tools set up.

Set Up Your Environment:

Set up the environment using cargo and install the wasm32-unknown-unknown target for WebAssembly:
```bash
rustup target add wasm32-unknown-unknown
```

Compile the Contract:

```bash
cargo wasm
```

Optimize the Contract: You can optimize the Wasm binary for deployment using wasm-opt:

```bash
wasm-opt -Oz target/wasm32-unknown-unknown/release/nft_contract.wasm -o nft_contract_optimized.wasm
```

Deploy the Contract:

You will need to deploy the contract using the sourced CLI. Assuming the sourced CLI is already set up:
```bash
sourced tx wasm store nft_contract_optimized.wasm --from <wallet-name> --gas auto --gas-adjustment 1.5 --fees 1000usource
```
The transaction will return a code ID.
Instantiate the Contract: After deploying, you need to instantiate the contract:

```bash
sourced tx wasm instantiate <code-id> '{"minter": "<minter-address>"}' --from <wallet-name> --label "nft-source-logo" --gas auto --fees 1000usource
```

This sets the address that will have permission to mint NFTs.

Mint the NFT: To mint the NFT with the specified image URL, use the following command:

```bash
sourced tx wasm execute <contract-address> '{"mint": {"token_id": "source-logo-1", "owner": "<owner-address>", "token_uri": "https://2352959449-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2F2ufmW7exCclo14ABKCcm%2Fuploads%2FTn4GWYaW1ZchQDYghUe0%2Fsource-logo.svg?alt=media&token=c53d45f9-a1a3-44f6-9730-c245e206e5ec"}}' --from <minter-address> --gas auto --fees 1000usource
```

This will mint the NFT for the specified owner and link it to the image URL.

