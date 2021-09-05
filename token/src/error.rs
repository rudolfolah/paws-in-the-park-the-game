use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Initial total supply must be greater than zero")]
    InitialTotalSupplyMustBeGreaterThanZero {},

    #[error("Amount to send is greater than the balance available")]
    AmountIsGreaterThanAvailableBalance {},

    #[error("Amount cannot be zero")]
    AmountCannotBeZero {},

    // TODO: the following need to go into a separate smart contract
    #[error("Dog with given id does not exist")]
    DogDoesNotExist {},

    #[error("Listing price is below minimum")]
    ListingPriceTooLow {},

    #[error("Listing price is above maximum")]
    ListingPriceTooHigh {},

    #[error("Object is already listed on market")]
    AlreadyListedOnMarket {},

    #[error("Listing with given id does not exist")]
    ListingDoesNotExist {},

    #[error("Does not match listing price")]
    DoesNotMatchListingPrice {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Failed commission calculation")]
    FailedCommissionCalculation {},
}
