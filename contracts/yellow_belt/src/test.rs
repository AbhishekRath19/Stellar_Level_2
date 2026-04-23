#![cfg(test)]
use super::*;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, YellowBeltContract);
    let client = YellowBeltContractClient::new(&env, &contract_id);

    assert_eq!(client.get_count(), 0);
    
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.get_count(), 2);

    client.reset();
    assert_eq!(client.get_count(), 0);
}
