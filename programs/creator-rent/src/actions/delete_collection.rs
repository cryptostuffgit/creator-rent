use crate::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteCollection<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = nft_collection.creator == payer.key(),
        close = payer
    )]
    pub nft_collection: Account<'info, NftCollection>,
    pub system_program: Program<'info, System>,    
}

impl DeleteCollection<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>) -> Result<()> {
        Ok(())
    }
}