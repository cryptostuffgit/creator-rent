use crate::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(create_collection_params: UpdateRentParams)]
pub struct UpdateRent<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = nft_collection.creator == payer.key()
    )]
    pub nft_collection: Account<'info, NftCollection>,
    pub system_program: Program<'info, System>,    
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateRentParams {
    new_rent_price: u64
}

impl UpdateRent<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &UpdateRentParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: UpdateRentParams) -> Result<()> {
        let nft_collection = &mut ctx.accounts.nft_collection;
        nft_collection.rent_price = params.new_rent_price;
        Ok(())
    }
}