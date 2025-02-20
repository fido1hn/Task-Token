use anchor_lang::{error, error_code};

#[error_code]
pub enum CustomError {
    #[msg("Invalid difficulty level")]
    InvalidDifficulty,
}
