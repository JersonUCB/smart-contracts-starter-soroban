use super::{State, Vault, VaultClient};
use soroban_sdk::{
    testutils::{Address as _, Events as _},
    Address, Env,
};

#[test]
fn only_admin_can_change_state() {
    let env = Env::default();
    let contract_id = env.register(Vault, ());
    let client = VaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.initialize(&admin);
    assert!(client.try_open().is_err());

    env.mock_all_auths();
    client.open();
    assert_eq!(env.events().all().events().len(), 1);
    assert_eq!(client.state(), State::Open);
}

#[test]
fn changing_admin_also_requires_authorization() {
    let env = Env::default();
    let contract_id = env.register(Vault, ());
    let client = VaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);

    client.initialize(&admin);
    assert!(client.try_change_admin(&new_admin).is_err());

    env.mock_all_auths();
    client.change_admin(&new_admin);
    assert_eq!(client.admin(), new_admin);
}
