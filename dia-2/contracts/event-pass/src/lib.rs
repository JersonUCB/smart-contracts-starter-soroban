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
    NotPurchased = 4,
    NotHolder = 5,
}

#[contract]
pub struct EventPass;

#[contractimpl]
impl EventPass {
    pub fn buy(env: Env, buyer: Address) -> Result<(), Error> {
        buyer.require_auth();

        let state = env
            .storage()
            .instance()
            .get(&DataKey::State)
            .unwrap_or(State::NotPurchased);

        if state != State::NotPurchased {
            return Err(Error::AlreadyPurchased);
        }

        env.storage().instance().set(&DataKey::Holder, &buyer);
        env.storage()
            .instance()
            .set(&DataKey::State, &State::Purchased);
        Ok(())
    }

    pub fn redeem(env: Env, holder: Address) -> Result<(), Error> {
        require_holder(&env, &holder)?;

        let state: State = env
            .storage()
            .instance()
            .get(&DataKey::State)
            .ok_or(Error::NotPurchased)?;

        if state == State::Redeemed {
            return Err(Error::AlreadyRedeemed);
        }
        if state != State::Purchased {
            return Err(Error::NotPurchased);
        }

        env.storage()
            .instance()
            .set(&DataKey::State, &State::Redeemed);
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

fn require_holder(env: &Env, holder: &Address) -> Result<(), Error> {
    let stored_holder: Address = env
        .storage()
        .instance()
        .get(&DataKey::Holder)
        .ok_or(Error::NotPurchased)?;
    holder.require_auth();

    if stored_holder != *holder {
        return Err(Error::NotHolder);
    }

    Ok(())
}

#[cfg(test)]
mod test;
