use std::borrow::BorrowMut;

use cosmwasm_std::{
    DepsMut, Empty, Response
};
use cw2::set_contract_version;

use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractError::{self}, state::{STATE, CONTRACT_CREATOR},
};


const FROM_VERSION: &str = "1.0.0";

pub mod v1_state {
    use cosmwasm_std::Addr;
    use cw_storage_plus::{Item, Map};

    /// The creator of the contract who can set the initial contract addresses. After that, only sudo can change them.
    pub const CONTRACT_CREATOR: Item<Addr> = Item::new("contract_creator");

    pub const STATE: Map<String, String> = Map::new("inputs");


}

pub fn migrate(deps: DepsMut) -> Result<Response, ContractError> {
    // make sure we're migrating the correct contract and from the correct version
    cw2::assert_contract_version(
        deps.as_ref().storage,
        &format!("crates.io:{CONTRACT_NAME}"),
        FROM_VERSION,
    )?;

    // migrate state
    STATE.clone_from(&v1_state::STATE);

    // clear old state
    v1_state::STATE.clear(deps.storage);

    
    let old_creator = v1_state::CONTRACT_CREATOR.load(deps.as_ref().storage)?; 

    // migrate creator
    CONTRACT_CREATOR.save(deps.storage, &old_creator)?;
    v1_state::CONTRACT_CREATOR.remove(deps.storage);


    // migrate version
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), "2.0.0")?;

    Ok(cw721_base::upgrades::v0_17::migrate::<Empty, Empty, Empty, Empty>(deps)?)
}

