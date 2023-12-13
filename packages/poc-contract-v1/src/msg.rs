use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    PostResults{results: Vec<String>},
    // //--------------------------------------------------------------------------------------------------
    // // Migrate message to work in batches
    // //--------------------------------------------------------------------------------------------------
    // Migrate(MigrateV1ToV2),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetStateSizeResponse)]
    GetStateSize{},

    #[returns(GetStateKeysResponse)]
    GetStateKeys{}

}



#[cw_serde]
pub enum PocSudoMsg {
    PostResults{results: Vec<String>},
    Infinite{},
    // //--------------------------------------------------------------------------------------------------
    // // Migrate message to work in batches
    // //--------------------------------------------------------------------------------------------------
    // Migrate(MigrateV1ToV2),

}



#[cw_serde]

pub struct GetStateSizeResponse{
    pub size: usize
}

#[cw_serde]

pub struct GetStateKeysResponse{
    pub keys: Vec<String>
}

// /// Migrate from V1 to V2
// #[cw_serde]
// pub enum MigrateV1ToV2 {
//     /// Burns empty accounts in batches
//     MigrateSate {
//     },
// }

#[cw_serde]
pub enum MigrateMsg {
    V1_0_0_to_V2_0_0{},
}