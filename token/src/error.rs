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
}
