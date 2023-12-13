use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    PostResults{results: Vec<String>}
}

#[cw_serde]
// #[derive(QueryResponses)]
pub enum QueryMsg {}



#[cw_serde]
pub enum PocSudoMsg {
    PostResults{results: Vec<String>},
    Infinite{}

}


