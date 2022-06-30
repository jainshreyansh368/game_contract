use solana_program::program_error::ProgramError;

use crate::error::MetalityGameContractError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetalityGameContractInstruction {
    InitializeGameProgramData,
    InitializeGame,
    CancelGame,
    JoinGame,
    TransferReward,
    DrawOrCancelGame,
    SetGameStarted,
    UserBExitGame,
}

impl MetalityGameContractInstruction {
    pub fn unpack_instruction_data(ins_data: &[u8]) -> Result<Self, ProgramError> {
        let (ins_no, _data) = ins_data
            .split_first()
            .ok_or(MetalityGameContractError::InvalidInstruction)?;

        Ok(match ins_no {
            0 => Self::InitializeGameProgramData,
            1 => Self::InitializeGame,
            2 => Self::CancelGame,
            3 => Self::JoinGame,
            4 => Self::TransferReward,
            5 => Self::DrawOrCancelGame,
            6 => Self::SetGameStarted,
            7 => Self::UserBExitGame,
            _ => return Err(MetalityGameContractError::InvalidInstruction.into()),
        })
    }
}
