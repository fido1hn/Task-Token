use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Invalid difficulty level")]
    InvalidDifficulty,
    #[msg("Account not a valid Submission PDA")]
    MismatchedAccount,
    #[msg("Account not owned by Task Token Protocol")]
    AccountNotOwnedByProgram,
}
