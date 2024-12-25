use anchor_lang::prelude::*;

#[error_code]
pub enum MintError {
    #[msg("Invalid fee value")]
    InvalidFee,

    #[msg("Invalid Recipient")]
    InvalidRecipient,
    #[msg("Invalid Fee Recipient")]
    InvalidFeeRecipient,

    #[msg("Invalid Id")]
    InvalidId,

    #[msg("Invalid Admin")]
    InvalidAdmin,

    #[msg("Invalid Amount")]
    InvalidAmount,

    #[msg("Invalid User Limit")]
    InvalidUserLimit,
    #[msg("Invalid Token")]
    InvalidToken,

    #[msg("Invalid Time")]
    InvalidTime,
}
