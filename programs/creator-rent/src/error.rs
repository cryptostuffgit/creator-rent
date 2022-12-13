use anchor_lang::prelude::*;

#[error_code]
pub enum RentError {
    #[msg("Sorry, you can only create a rent account for an NFT collection that you have update authority over")]
    NotUpdateAuthority,
    #[msg("Sorry, you can't pay for more than 64 periods")]
    TooManyPeriodsPaid,
    #[msg("Sorry, please use the address of an NFT that's part of the collection")]
    NftNotPartOfCollection,
    #[msg("Restricted! You are not a manager.")]
    NotAManager,
    #[msg("Please make sure your rent price is greater than 0")]
    RentNotGreaterThanZero,
    #[msg("Please make sure your period is greater than 0")]
    PeriodNotGreaterThanZero,
    #[msg("You cannot collect rent for this collection")]
    NotCollectionCreator,
}
