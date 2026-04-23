#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Counter,
}

#[contract]
pub struct YellowBeltContract;

#[contractimpl]
impl YellowBeltContract {
    /// Increment the counter and store the new value
    pub fn increment(env: Env) -> u32 {
        let key = DataKey::Counter;
        
        // Read current value (default to 0 if not found)
        let mut count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        
        // Increment
        count += 1;
        
        // Store the new value
        env.storage().instance().set(&key, &count);
        
        // Extend TTL to keep the data alive
        env.storage().instance().extend_ttl(100, 1000);
        
        // Emit an event
        env.events().publish((symbol_short!("counter"), symbol_short!("inc")), count);
        
        log!(&env, "Counter incremented to: {}", count);
        
        count
    }

    /// Decrement the counter and store the new value
    pub fn decrement(env: Env) -> u32 {
        let key = DataKey::Counter;
        
        let mut count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        
        if count > 0 {
            count -= 1;
        }
        
        env.storage().instance().set(&key, &count);
        env.storage().instance().extend_ttl(100, 1000);
        
        env.events().publish((symbol_short!("counter"), symbol_short!("dec")), count);
        
        count
    }

    /// Read the current counter value
    pub fn get_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Counter).unwrap_or(0)
    }

    /// Reset the counter to zero (write function)
    pub fn reset(env: Env) {
        let key = DataKey::Counter;
        env.storage().instance().set(&key, &0u32);
        
        // Emit event
        env.events().publish((symbol_short!("counter"), symbol_short!("reset")), 0u32);
    }
}

mod test;
