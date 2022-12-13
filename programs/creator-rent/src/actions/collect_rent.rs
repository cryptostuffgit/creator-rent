
use crate::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct CollectRent<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,
    #[account(
        mut,
        constraint = nft_collection.creator == payer.key()
        @ RentError::NotCollectionCreator,
        seeds = [b"collection", collection_mint.key().as_ref()],
        bump,
    )]
    pub nft_collection: Account<'info, NftCollection>,
    pub collection_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,  
}


impl CollectRent<'_>{
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()>{
        let nft_collection = &mut ctx.accounts.nft_collection;
        let payer = &mut ctx.accounts.payer;
        
        let now = Clock::get().unwrap().unix_timestamp as u32;
        let cycles_to_collect = (now - nft_collection.last_time_collected) / (nft_collection.period_in_seconds as u32);
        let mut amount_to_collect: u64 = 0;
        let current_index = nft_collection.rent_collection_index as u32;
        for i in 0..=cycles_to_collect {
            amount_to_collect += nft_collection.rent_collection[(i + current_index) as usize];
            nft_collection.rent_collection[(i + current_index) as usize] = 0;
        }
        nft_collection.rent_collection_index = ((current_index + cycles_to_collect) % 64) as u8;

        let mut payer_lamports = payer.lamports.borrow_mut();
        let nft_collection_info = nft_collection.to_account_info();
        let mut collection_lamports = nft_collection_info.lamports.borrow_mut();
        
        **payer_lamports += amount_to_collect;
        **collection_lamports -= amount_to_collect;

        nft_collection.last_time_collected = now;

        Ok(())
    }
}