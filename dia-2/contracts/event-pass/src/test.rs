use super::{Error, EventPass, EventPassClient, State};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn buy_requires_buyer_authorization() {
    let env = Env::default();
    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);
    let buyer = Address::generate(&env);

    assert!(client.try_buy(&buyer).is_err());

    env.mock_all_auths();
    client.buy(&buyer);
    assert_eq!(client.holder(), buyer);
    assert_eq!(client.state(), State::Purchased);
}

#[test]
fn buy_cannot_be_performed_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);
    let first_buyer = Address::generate(&env);
    let second_buyer = Address::generate(&env);

    client.buy(&first_buyer);

    assert_eq!(
        client.try_buy(&second_buyer),
        Err(Ok(Error::AlreadyPurchased))
    );
    assert_eq!(client.holder(), first_buyer);
    assert_eq!(client.state(), State::Purchased);
}

#[test]
fn redeem_succeeds_after_buy() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);
    let holder = Address::generate(&env);

    client.buy(&holder);
    client.redeem(&holder);

    assert_eq!(client.state(), State::Redeemed);
}

#[test]
fn redeem_requires_holder_authorization() {
    let env = Env::default();
    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);
    let holder = Address::generate(&env);

    env.mock_all_auths();
    client.buy(&holder);
    env.set_auths(&[]);

    assert!(client.try_redeem(&holder).is_err());
}

#[test]
fn only_the_stored_holder_can_redeem() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);
    let holder = Address::generate(&env);
    let other_address = Address::generate(&env);

    client.buy(&holder);

    assert_eq!(client.try_redeem(&other_address), Err(Ok(Error::NotHolder)));
    assert_eq!(client.state(), State::Purchased);
}

#[test]
fn redeem_before_buy_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);
    let holder = Address::generate(&env);

    assert_eq!(client.try_redeem(&holder), Err(Ok(Error::NotPurchased)));
}

#[test]
fn redeem_cannot_be_performed_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);
    let holder = Address::generate(&env);

    client.buy(&holder);
    client.redeem(&holder);

    assert_eq!(client.try_redeem(&holder), Err(Ok(Error::AlreadyRedeemed)));
}
