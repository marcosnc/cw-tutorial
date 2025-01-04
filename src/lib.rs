use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

mod contract;
mod error;
mod msg;
mod state;

#[entry_point]
pub fn instantiate(
  deps: DepsMut, 
  env: Env, 
  info: MessageInfo, 
  msg: msg::InstantiateMsg
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
  deps: DepsMut, 
  env: Env, 
  info: MessageInfo, 
  msg: msg::ExecuteMsg
) -> Result<Response, error::ContractError> {
    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(
  deps: Deps, 
  env: Env, 
  msg: msg::QueryMsg
) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}
