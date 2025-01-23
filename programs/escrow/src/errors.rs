use anchor_lang::prelude::*;

#[error_code]
pub enum MakeErrors {
    #[msg("Transfer failed")]
    TransferFailed,
}
