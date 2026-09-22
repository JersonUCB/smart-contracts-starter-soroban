#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env,
};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Closed,
    Open,
}

#[contracttype]
enum DataKey {
    Admin,
    State,
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    StateUnchanged = 3,
}

#[contractevent(topics = ["state"])]
pub struct StateChanged {
    pub state: State,
}

#[contractevent(topics = ["admin"])]
pub struct AdminChanged {
    pub new_admin: Address,
}

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::State, &State::Closed);
        Ok(())
    }

    pub fn open(env: Env) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        change_state(&env, State::Open)
    }

    pub fn close(env: Env) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        change_state(&env, State::Closed)
    }

    pub fn change_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NotInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &new_admin);
        AdminChanged { new_admin }.publish(&env);
        Ok(())
    }

    pub fn admin(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    pub fn state(env: Env) -> Result<State, Error> {
        env.storage()
            .instance()
            .get(&DataKey::State)
            .ok_or(Error::NotInitialized)
    }
}

fn change_state(env: &Env, new_state: State) -> Result<(), Error> {
    let current_state: State = env
        .storage()
        .instance()
        .get(&DataKey::State)
        .ok_or(Error::NotInitialized)?;

    if current_state == new_state {
        return Err(Error::StateUnchanged);
    }

    env.storage().instance().set(&DataKey::State, &new_state);
    StateChanged { state: new_state }.publish(env);
    Ok(())
}

#[cfg(test)]
mod test;
