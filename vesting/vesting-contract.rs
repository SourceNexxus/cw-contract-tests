use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Contract name and version
const CONTRACT_NAME: &str = "vesting-contract";
const CONTRACT_VERSION: &str = "0.2.0";
const TOTAL_AMOUNT: Uint128 = Uint128::new(1_000_000_000_000); // 1,000,000 uSource in uSource (assuming 6 decimal places)
const VESTING_DURATION: u64 = 360 * 24 * 60 * 60; // 360 days in seconds
const RELEASE_PERIOD: u64 = 24 * 60 * 60; // 1 day in seconds

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub beneficiary: String, // Beneficiary's address
    pub owner: String,       // Owner's address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Claim {},
    Pause {},
    Unpause {},
    Revoke {},
    Withdraw { amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub beneficiary: Addr,
    pub start_time: u64,
    pub claimed: Uint128,
    pub paused: bool,
    pub revoked: bool,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        owner: deps.api.addr_validate(&msg.owner)?,
        beneficiary: deps.api.addr_validate(&msg.beneficiary)?,
        start_time: env.block.time.seconds(),
        claimed: Uint128::zero(),
        paused: false,
        revoked: false,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    deps.storage.set(b"state", &bincode::serialize(&state)?);

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Claim {} => try_claim(deps, env, info),
        ExecuteMsg::Pause {} => try_pause(deps, info),
        ExecuteMsg::Unpause {} => try_unpause(deps, info),
        ExecuteMsg::Revoke {} => try_revoke(deps, info),
        ExecuteMsg::Withdraw { amount } => try_withdraw(deps, info, amount),
    }
}

// Pauses the vesting process
pub fn try_pause(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut state: State = bincode::deserialize(&deps.storage.get(b"state").unwrap())
        .map_err(|_| StdError::generic_err("Failed to load state"))?;

    if info.sender != state.owner {
        return Err(StdError::generic_err("Unauthorized"));
    }

    state.paused = true;
    deps.storage.set(b"state", &
