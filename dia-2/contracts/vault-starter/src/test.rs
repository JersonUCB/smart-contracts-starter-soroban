use super::{Error, State, Vault, VaultClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn controls_state_and_errors() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Vault, ());
    let client = VaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.initialize(&admin);
    assert_eq!(client.state(), State::Closed);
    assert_eq!(client.try_close(), Err(Ok(Error::StateUnchanged)));
    client.open();
    assert_eq!(client.state(), State::Open);
}

#[test]
fn challenge_anyone_can_replace_the_admin() {
    let env = Env::default();
    let contract_id = env.register(Vault, ());
    let client = VaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let attacker = Address::generate(&env);

    client.initialize(&admin);
    client.change_admin(&attacker);

    assert_eq!(client.admin(), attacker);
}
