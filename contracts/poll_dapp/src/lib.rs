#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Votes,     // Map<u32, u32> - Option index to vote count
    Voters,    // Map<Address, bool> - Voter address to voted status
}

#[contract]
pub struct PollContract;

#[contractimpl]
impl PollContract {
    /// Vote for an option (0-index)
    pub fn vote(env: Env, voter: Address, option: u32) {
        voter.require_auth();

        let mut voters: Map<Address, bool> = env.storage().instance().get(&DataKey::Voters).unwrap_or(Map::new(&env));
        
        if voters.contains_key(voter.clone()) {
            panic!("Address has already voted");
        }

        let mut votes: Map<u32, u32> = env.storage().instance().get(&DataKey::Votes).unwrap_or(Map::new(&env));
        let current_votes = votes.get(option).unwrap_or(0);
        votes.set(option, current_votes + 1);

        voters.set(voter.clone(), true);

        env.storage().instance().set(&DataKey::Votes, &votes);
        env.storage().instance().set(&DataKey::Voters, &voters);
        
        // Extend TTL
        env.storage().instance().extend_ttl(100, 1000);

        // Emit event
        env.events().publish(
            (symbol_short!("poll"), symbol_short!("vote")),
            (voter, option),
        );
    }

    /// Get current results
    pub fn get_results(env: Env) -> Map<u32, u32> {
        env.storage().instance().get(&DataKey::Votes).unwrap_or(Map::new(&env))
    }

    /// Check if an address has voted
    pub fn has_voted(env: Env, voter: Address) -> bool {
        let voters: Map<Address, bool> = env.storage().instance().get(&DataKey::Voters).unwrap_or(Map::new(&env));
        voters.contains_key(voter)
    }
}
