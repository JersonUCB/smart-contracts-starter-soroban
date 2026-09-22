#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Total, // total -> 5
    Visits(Address), // GFABIAN... -> 5
}

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn greet(env: Env, visitor: Address) -> u32 {
        let total: u32 = env.storage().instance().get(&DataKey::Total).unwrap_or(0);
        let visits: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::Visits(visitor.clone()))
            .unwrap_or(0);

        let new_total = total.checked_add(1).expect("total overflow");
        let new_visits = visits.checked_add(1).expect("visits overflow");

        env.storage().instance().set(&DataKey::Total, &new_total);
        env.storage()
            .persistent()
            .set(&DataKey::Visits(visitor), &new_visits);

        new_visits
    }

    pub fn total(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Total).unwrap_or(0)
    }

    pub fn visits(env: Env, visitor: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Visits(visitor))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
