use super::{Counter, CounterClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn counts_visits_per_person_and_in_total() {
    let env = Env::default();
    let contract_id = env.register(Counter, ());
    let client = CounterClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    assert_eq!(client.total(), 0);
    assert_eq!(client.greet(&alice), 1);
    assert_eq!(client.greet(&alice), 2);
    assert_eq!(client.greet(&bob), 1);
    assert_eq!(client.visits(&alice), 2);
    assert_eq!(client.visits(&bob), 1);
    assert_eq!(client.total(), 3);
}

#[test]
fn a_new_address_starts_at_zero() {
    let env = Env::default();
    let contract_id = env.register(Counter, ());
    let client = CounterClient::new(&env, &contract_id);
    let visitor = Address::generate(&env);

    assert_eq!(client.visits(&visitor), 0);
}
