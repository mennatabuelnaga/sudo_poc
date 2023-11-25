use crate::tests::utils::{proper_instantiate, generate_random_string_vector, ADMIN};

use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use poc_contract::msg::{ExecuteMsg, PocSudoMsg};


const INPUT_SIZE: usize = 2_000_000;


#[test]
fn sudo_post() {
    let (mut app, poc_contract) = proper_instantiate();

   
    let results: Vec<String> = generate_random_string_vector(INPUT_SIZE, 10);
   

    // sudo post
    let msg = PocSudoMsg::PostResults { results };
    let cosmos_msg = poc_contract.sudo(msg);
    app.sudo(cosmos_msg).unwrap();

   
}



#[test]
fn execute_post() {

    let (mut app, poc_contract) = proper_instantiate();

   
    let results: Vec<String> = generate_random_string_vector(INPUT_SIZE, 10);
 

    // execute post
    let msg = ExecuteMsg::PostResults { results } ;
    let cosmos_msg = poc_contract.call(msg).unwrap();
    app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();
  
  
}
