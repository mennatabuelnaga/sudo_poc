use cosmwasm_std::to_json_binary;
use cosmwasm_std::{Addr, Coin, CosmosMsg, Empty, StdResult, Uint128, WasmMsg};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};


pub const USER: &str = "user";
pub const EXECUTOR_1: &str = "executor1";
pub const EXECUTOR_2: &str = "executor2";
pub const EXECUTOR_3: &str = "executor3";
pub const ADMIN: &str = "admin";
pub const NATIVE_DENOM: &str = "seda";

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<poc_contract::msg::ExecuteMsg>>(
        &self,
        msg: T,
    ) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

   

    pub fn sudo<T: Into<poc_contract::msg::PocSudoMsg>>(
        &self,
        msg: T,
    ) -> cw_multi_test::SudoMsg {
        let msg = to_json_binary(&msg.into()).unwrap();
        cw_multi_test::SudoMsg::Wasm(cw_multi_test::WasmSudo {
            contract_addr: self.addr().into(),
            msg,
        })
    }

   
}

pub fn poc_contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        poc_contract::contract::execute,
        poc_contract::contract::instantiate,
        poc_contract::contract::query,
    )
    .with_sudo(poc_contract::contract::sudo);
    Box::new(contract)
}





fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(USER),
                vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(100),
                }],
            )
            .unwrap();
    })
}

pub fn proper_instantiate() -> (App, CwTemplateContract) {
    let mut app = mock_app();

    // instantiate poc-contract
    let poc_contract_template_id = app.store_code(poc_contract_template());
    let msg = poc_contract::msg::InstantiateMsg {
    };
    let poc_contract_addr = app
        .instantiate_contract(
            poc_contract_template_id,
            Addr::unchecked(ADMIN),
            &msg,
            &[],
            "test",
            None,
        )
        .unwrap();
    let poc_template_contract = CwTemplateContract(poc_contract_addr.clone());

    (app, poc_template_contract)
}



fn generate_random_string(length: usize) -> String {
    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    random_string
}

pub fn generate_random_string_vector(size: usize, string_length: usize) -> Vec<String> {
    (0..size)
        .map(|_| generate_random_string(string_length))
        .collect()
}

