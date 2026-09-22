#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    NotPurchased,
    Purchased,
    Redeemed,
}

#[contracttype]
enum DataKey {
    Holder,
    State,
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyPurchased = 2,
    AlreadyRedeemed = 3,
}

#[contract]
pub struct EventPass;

#[contractimpl]
impl EventPass {
    pub fn buy(env: Env, buyer: Address) -> Result<(), Error> {
        let state = env
            .storage()
            .instance()
            .get(&DataKey::State)
            .unwrap_or(State::NotPurchased);

        if state != State::NotPurchased {
            return Err(Error::AlreadyPurchased);
        }

        buyer.require_auth();
        env.storage().instance().set(&DataKey::Holder, &buyer);
        env.storage()
            .instance()
            .set(&DataKey::State, &State::Purchased);
        Ok(())
    }

    pub fn holder(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Holder)
            .ok_or(Error::NotInitialized)
    }

    pub fn state(env: Env) -> Result<State, Error> {
        env.storage()
            .instance()
            .get(&DataKey::State)
            .ok_or(Error::NotInitialized)
    }
}

#[cfg(test)]
mod test;
