use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MetalityGameContractError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Incorrect PDA")]
    IncorrectPDA,

    #[error("User nor signer")]
    UserNotSigner,

    #[error("Incorrect token ATA Owner")]
    IncorrectATAOwner,

    #[error("Equality Mismatch")]
    EqualityMismatch,

    #[error("Invalid token ata, mint mismatch")]
    InvalidTokenATA,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Not Admin")]
    NotAdmin,

    #[error("Same user joining game")]
    SameUserJoining,

    #[error("Invalid Winner")]
    InvalidWinner,

    #[error("Invalid State Account")]
    InvalidStateAccount,
}

impl From<MetalityGameContractError> for ProgramError {
    fn from(e: MetalityGameContractError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
