use crate::*;
use anchor_lang::prelude::*;
use anchor_lang::prelude::Clock;
use anchor_spl::token::Mint;
use mpl_token_metadata::{
    ID,
    state::{Metadata, TokenMetadataAccount}
};

#[derive(Accounts)]
#[instruction(create_collection_params: CreateCollectionParams)]
pub struct CreateCollection<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = NftCollection::NFT_COLLECTION_SIZE,
        seeds = [b"collection", collection_mint.key().as_ref()],
        bump,
    )]
    pub nft_collection: Account<'info, NftCollection>,
    pub collection_mint: Account<'info, Mint>,
    
    #[account(        
        seeds = [b"metadata", ID.as_ref(), collection_mint.key().as_ref()],
        seeds::program = ID,
        bump
    )]
    /// CHECK: This is not dangerous
    pub collection_metadata: AccountInfo<'info>,
    pub system_program: Program<'info, System>,    
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionParams {
    period_in_seconds: u32,
    rent_price: u64
}

impl CreateCollection<'_> {
    pub fn validate(&self, ctx: &Context<Self>, params: &CreateCollectionParams) -> Result<()> {
        let collection_metadata_info = ctx.accounts.collection_metadata.to_account_info();        
        let metadata = Metadata::from_account_info(&collection_metadata_info).unwrap();
        require!(metadata.update_authority == ctx.accounts.payer.key(), RentError::NotUpdateAuthority);
        
        require!(params.rent_price > 0, RentError::RentNotGreaterThanZero);
        require!(params.period_in_seconds > 0, RentError::RentNotGreaterThanZero);
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: CreateCollectionParams) -> Result<()> {
        let nft_collection = &mut ctx.accounts.nft_collection;
        let payer = &mut ctx.accounts.payer;
        
        nft_collection.creator = payer.key();
        nft_collection.collection_mint = ctx.accounts.collection_mint.key();
        nft_collection.time_created = Clock::get().unwrap().unix_timestamp as u32;
        nft_collection.period_in_seconds = params.period_in_seconds;
        nft_collection.last_time_collected = Clock::get().unwrap().unix_timestamp as u32;
        nft_collection.rent_price = params.rent_price;
        nft_collection.rent_collection_index = 0;

        Ok(())
    }
}