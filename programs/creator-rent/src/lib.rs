pub mod actions;
pub mod error;
pub mod state;
pub use actions::*;
use {
    crate::{error::*, state::*},
};
use anchor_lang::prelude::*;

declare_id!("5Fq8padZgVoCsLBE5N6RsAmmJVpwRRqLVD6ysX36pjsS");

#[program]
pub mod creator_rent {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn create_collection(ctx: Context<CreateCollection>, params: CreateCollectionParams) -> Result<()> {
        CreateCollection::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn pay_rent(ctx: Context<PayRent>, params: PayRentParams) -> Result<()> {
        PayRent::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn update_rent(ctx: Context<UpdateRent>, params: UpdateRentParams) -> Result<()> {
        UpdateRent::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn collect_rent(ctx: Context<CollectRent>) -> Result<()> {
        CollectRent::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn delete_collection(ctx: Context<DeleteCollection>) -> Result<()> {
        DeleteCollection::actuate(ctx)
    }
}