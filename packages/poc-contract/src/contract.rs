#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, PocSudoMsg};
use crate::state::{INPUTS, CONTRACT_CREATOR};
use sha3::{Digest, Keccak256};

// version info
const CONTRACT_NAME: &str = "poc-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CONTRACT_CREATOR.save(deps.storage, &info.sender)?;
    Ok(Response::new().add_attribute("method", "instantiate"))


}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Admin
        ExecuteMsg::PostResults { results } => {
            for input in results {
                let hash = hash_string(input.clone());
                INPUTS.save(deps.storage, hash, &input)?;

            }

            Ok(Response::new().add_attribute("method", "post_result"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: PocSudoMsg) -> Result<Response, ContractError> {
    match msg {
        PocSudoMsg::PostResults { results } => {
            for input in results {
                let hash = hash_string(input.clone());
                INPUTS.save(deps.storage, hash, &input)?;

            }
            Ok(Response::new().add_attribute("method", "post_result"))

        }
    }
}

pub fn hash_string(input: String) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(input);

    format!("0x{}", hex::encode(hasher.finalize()))}


#[cfg(test)]
mod tests {

}
