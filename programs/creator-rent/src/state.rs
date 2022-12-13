use anchor_lang::prelude::*;

#[account]
pub struct NftCollection {
    pub creator: Pubkey,
    pub collection_mint: Pubkey,
    pub time_created: u32,
    pub period_in_seconds: u32,
    pub last_time_collected: u32,
    pub rent_price: u64,
    pub rent_collection_index: u8,
    pub rent_collection: [u64; 64]
}

impl NftCollection {
    pub const NFT_COLLECTION_SIZE: usize = 8 + 32 + 32 + 4 + 4 + 4 + 8 + 1 + 64 * 8; 
}

#[account]
pub struct NftRent {
    pub rent_price: u64,
    pub active_until: u32,
}

impl NftRent {
    pub const NFT_RENT_SIZE: usize = 8 + 8 + 4; 
}