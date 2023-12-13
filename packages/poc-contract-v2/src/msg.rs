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
    GetStateKeys{},

    #[returns(GetMigrationMsgResponse)]
    GetMigrationMsg{}

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


#[cw_serde]
pub enum MigrateMsg {
    V2_0_0ToV3_0_0{},
}

#[cw_serde]

pub struct GetMigrationMsgResponse{
    pub msg: String
}