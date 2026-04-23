#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub owner: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    NFT(u32),
    TotalSupply,
}

#[contract]
pub struct NFTContract;

#[contractimpl]
impl NFTContract {
    /// Mint a new NFT with metadata
    pub fn mint(env: Env, to: Address, name: String, description: String, image_url: String) -> u32 {
        // Get and increment total supply (token ID)
        let mut total_supply: u32 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        total_supply += 1;

        let metadata = NFTMetadata {
            name,
            description,
            image_url,
            owner: to.clone(),
        };

        // Store metadata
        env.storage().instance().set(&DataKey::NFT(total_supply), &metadata);
        env.storage().instance().set(&DataKey::TotalSupply, &total_supply);
        
        // Extend TTL
        env.storage().instance().extend_ttl(100, 1000);

        // Emit event
        env.events().publish(
            (symbol_short!("nft"), symbol_short!("mint")),
            (total_supply, to),
        );

        total_supply
    }

    /// Get metadata for a specific NFT
    pub fn get_nft(env: Env, token_id: u32) -> Option<NFTMetadata> {
        env.storage().instance().get(&DataKey::NFT(token_id))
    }

    /// Get total supply of NFTs
    pub fn total_supply(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }
}
