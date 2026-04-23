#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    TotalAmount,
    GoalAmount,
    Deadline,
}

#[contract]
pub struct CrowdfundContract;

#[contractimpl]
impl CrowdfundContract {
    /// Initialize the crowdfund with a goal
    pub fn initialize(env: Env, goal: u32) {
        if env.storage().instance().has(&DataKey::GoalAmount) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::GoalAmount, &goal);
        env.storage().instance().set(&DataKey::TotalAmount, &0u32);
    }

    /// Donate an amount (simplified, as actual token transfer requires token client)
    pub fn donate(env: Env, donor: Address, amount: u32) -> u32 {
        donor.require_auth();

        let mut total: u32 = env.storage().instance().get(&DataKey::TotalAmount).unwrap_or(0);
        total += amount;

        env.storage().instance().set(&DataKey::TotalAmount, &total);
        
        // Extend TTL
        env.storage().instance().extend_ttl(100, 1000);

        // Emit event
        env.events().publish(
            (symbol_short!("fund"), symbol_short!("donate")),
            (donor, amount),
        );

        total
    }

    /// Get current total
    pub fn get_total(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::TotalAmount).unwrap_or(0)
    }

    /// Get goal amount
    pub fn get_goal(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::GoalAmount).unwrap_or(0)
    }
}
