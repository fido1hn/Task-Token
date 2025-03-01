use anchor_lang::error_code;

#[error_code]
pub enum TaskTokenError {
    #[msg("Invalid difficulty level")]
    InvalidDifficulty,
    #[msg("Account not a valid Submission PDA")]
    MismatchedAccount,
    #[msg("Account not owned by Task Token Protocol")]
    AccountNotOwnedByProgram,
    #[msg("The title is too long, it should be less than 50 characters")]
    TitleTooLong,
    #[msg("The description is too long, it should be less than 50 characters")]
    DescriptionTooLong,
}
