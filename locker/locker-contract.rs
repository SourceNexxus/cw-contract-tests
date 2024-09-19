use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Contract version info
const CONTRACT_NAME: &str = "locker-contract";
const CONTRACT_VERSION: &str = "0.1.0";

// Lock period (1 year in seconds)
const LOCK_PERIOD: u64 = 365 * 24 * 60 * 60;

// Locked amount (1,000,000 uSource, assuming 6 decimal places)
const LOCK_AMOUNT: Uint128 = Uint128::new(1_000_000_000_000); // 1,000,000 usource in micro units

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String, // Address of the token owner
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExecuteMsg {
    pub withdraw: Option<()>,  // Withdraw tokens if lock period has passed
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub lock_start_time: u64,  // Time when the tokens were locked
    pub locked: bool,          // Whether tokens are locked
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let owner = deps.api.addr_validate(&msg.owner)?;

    let state = State {
        owner,
        lock_start_time: env.block.time.seconds(),  // Current block time when locking starts
        locked: true,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    deps.storage.set(b"state", &bincode::serialize(&state)?);

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner)
        .add_attribute("locked_amount", LOCK_AMOUNT.to_string())
        .add_attribute("lock_start_time", state.lock_start_time.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg { withdraw: Some(_) } => try_withdraw(deps, env, info),
        _ => Err(StdError::generic_err("Unknown message")),
    }
}

// Handles the withdrawal of locked tokens after the lock period
fn try_withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let mut state: State = bincode::deserialize(&deps.storage.get(b"state").unwrap())
        .map_err(|_| StdError::generic_err("Failed to load state"))?;

    // Ensure that only the owner can withdraw
    if info.sender != state.owner {
        return Err(StdError::generic_err("Unauthorized: Only the owner can withdraw"));
    }

    // Calculate how much time has passed
    let elapsed_time = env.block.time.seconds() - state.lock_start_time;

    // Ensure that the lock period (1 year) has passed
    if elapsed_time < LOCK_PERIOD {
        return Err(StdError::generic_err("Lock period has not ended yet"));
    }

    // Check if tokens are still locked
    if !state.locked {
        return Err(StdError::generic_err("Tokens have already been withdrawn"));
    }

    // Unlock the tokens and mark them as withdrawn
    state.locked = false;
    deps.storage.set(b"state", &bincode::serialize(&state)?);

    // Create a message to transfer the locked tokens to the owner
    let send_msg = BankMsg::Send {
        to_address: state.owner.to_string(),
        amount: vec![Coin {
            denom: "usource".to_string(),
            amount: LOCK_AMOUNT,
        }],
    };

    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("method", "withdraw")
        .add_attribute("withdrawn_amount", LOCK_AMOUNT.to_string()))
}

// Allows querying the state of the contract (for debugging or status checks)
#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    let state: State = bincode::deserialize(&deps.storage.get(b"state").unwrap())
        .map_err(|_| StdError::generic_err("Failed to load state"))?;

    to_binary(&state)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryMsg {}  // Query for contract state
