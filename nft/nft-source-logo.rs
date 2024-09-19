use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use cw721_base::{ContractError, Cw721Contract, MintMsg};

const CONTRACT_NAME: &str = "nft-source-logo";
const CONTRACT_VERSION: &str = "0.1.0";

// Define a simple state for tracking the minter
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub minter: String,
}

// Instantiate the contract and set the minter
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let minter = deps.api.addr_validate(&msg.minter)?;
    let contract = Cw721Contract::default();
    contract.instantiate(
        deps,
        _env,
        info,
        cw721_base::msg::InstantiateMsg {
            name: "Source Logo NFT".to_string(),
            symbol: "SRCLOGO".to_string(),
            minter: minter.to_string(),
        },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("minter", minter))
}

// Execute: Mint a new NFT with the image as the metadata
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: cw721_base::msg::ExecuteMsg<MintMsg<Empty>>,
) -> Result<Response, ContractError> {
    let contract = Cw721Contract::default();
    contract.execute(deps, env, info, msg)
}

// Query the contract for NFT details
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: cw721_base::msg::QueryMsg) -> StdResult<Binary> {
    let contract = Cw721Contract::default();
    contract.query(deps, env, msg)
}
