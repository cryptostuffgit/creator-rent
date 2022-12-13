use crate::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use mpl_token_metadata::{
    ID,
    state::{Metadata, TokenMetadataAccount}
};

#[derive(Accounts)]
#[instruction(pay_rent_params: PayRentParams)]
pub struct PayRent<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,
    #[account(
        mut
    )]
    pub nft_collection: Account<'info, NftCollection>,
    pub nft_mint: Account<'info, Mint>,
    
    #[account(        
        seeds = [b"metadata", ID.as_ref(), nft_mint.key().as_ref()],
        seeds::program = ID,
        bump
    )]
    /// CHECK: This is not dangerous
    pub nft_metadata: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = NftRent::NFT_RENT_SIZE,
        seeds = [b"nft", nft_mint.key().as_ref()],
        bump
    )]
    pub nft_rent: Account<'info, NftRent>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PayRentParams {
    payment: u64
}


impl PayRent<'_>{
    pub fn validate(&self, ctx: &Context<Self>, params: &PayRentParams) -> Result<()>{
        let now = Clock::get().unwrap().unix_timestamp as u32;

        let rent_price: u64 = if ctx.accounts.nft_rent.rent_price > 0 {
            ctx.accounts.nft_rent.rent_price
        } else {
            ctx.accounts.nft_collection.rent_price
        };

        let pre_paid_offset: u64 = if ctx.accounts.nft_rent.active_until < now {
            0
        } else {
            (ctx.accounts.nft_rent.active_until - now) as u64 / ctx.accounts.nft_collection.period_in_seconds as u64
        };

        msg!("rent price {:?}", rent_price);
        msg!("pre_paid_offset {:?}", pre_paid_offset);
        
        require!(
            (params.payment / rent_price) + pre_paid_offset < 64, 
            RentError::TooManyPeriodsPaid,
        );

        let nft_metadata_info = ctx.accounts.nft_metadata.to_account_info();
        let metadata = Metadata::from_account_info(&nft_metadata_info).unwrap();
        require!(metadata.collection.unwrap().key == ctx.accounts.nft_collection.collection_mint, RentError::NftNotPartOfCollection);

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: PayRentParams) -> Result<()>{
        let nft_collection = &mut ctx.accounts.nft_collection;
        let nft_rent = &mut ctx.accounts.nft_rent;
        let payer = &mut ctx.accounts.payer;

        let now = Clock::get().unwrap().unix_timestamp as u32;

        if nft_rent.rent_price == 0 || nft_rent.active_until < now {
            nft_rent.rent_price = nft_collection.rent_price;
        }

        let rent_periods = params.payment / nft_rent.rent_price;
        let pre_paid_offset: u64 = if nft_rent.active_until < now {
            0
        } else {
            (nft_rent.active_until - now) as u64 / nft_collection.period_in_seconds as u64 + 1
        };
        
        let current_index = nft_collection.rent_collection_index as u64;
        for i in 0..rent_periods {
            nft_collection.rent_collection[(i + pre_paid_offset + current_index % 128) as usize] += nft_rent.rent_price;
        }

        if now < nft_rent.active_until {
            nft_rent.active_until += rent_periods as u32 * nft_collection.period_in_seconds;
        } else {
            nft_rent.active_until = now + (rent_periods as u32 * nft_collection.period_in_seconds);
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &payer.key(),
            &nft_collection.key(),
            params.payment,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                payer.to_account_info(),
                nft_collection.to_account_info(),
            ],
        )?;
        Ok(())
    }
}