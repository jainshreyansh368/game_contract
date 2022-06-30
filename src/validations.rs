use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use spl_token;

use crate::error::MetalityGameContractError;
use crate::state::MetalityGameContractState;

pub mod admin {
    solana_program::declare_id!("3KBgdH5xuVWKVB85L3SaRAiHXhDb77yd9qc6rxpNL2hr");
}

pub struct Validator;

impl Validator {
    pub fn validate_token_owner(
        token_ata: &AccountInfo,
        user: &AccountInfo,
    ) -> Result<(), ProgramError> {
        let token_ata_unpacked = spl_token::state::Account::unpack(&token_ata.try_borrow_data()?)?;

        if token_ata_unpacked.owner != *user.key {
            return Err(MetalityGameContractError::IncorrectATAOwner.into());
        }

        Ok(())
    }

    pub fn validate_token_ata(
        token_ata: &AccountInfo,
        token_mint: &AccountInfo,
    ) -> Result<(), ProgramError> {
        let token_ata_unpacked = spl_token::state::Account::unpack(&token_ata.try_borrow_data()?)?;

        let token_mint_unpacked = spl_token::state::Mint::unpack(&token_mint.try_borrow_data()?)?;

        if token_ata_unpacked.amount != 1
            || token_mint_unpacked.decimals != 0
            || token_ata_unpacked.mint != *token_mint.key
            || *token_ata.owner != spl_token::ID
        {
            return Err(MetalityGameContractError::InvalidTokenATA.into());
        }

        Ok(())
    }

    pub fn validate_is_signer(signer: &AccountInfo) -> Result<(), ProgramError> {
        if !signer.is_signer {
            return Err(MetalityGameContractError::UserNotSigner.into());
        }

        Ok(())
    }

    pub fn validate_admin(admin: &AccountInfo) -> Result<(), ProgramError> {
        if !admin.is_signer || *admin.key != admin::id() {
            return Err(MetalityGameContractError::NotAdmin.into());
        }

        Ok(())
    }

    pub fn validate_equality(lt: Pubkey, rt: Pubkey) -> Result<(), ProgramError> {
        if lt != rt {
            return Err(MetalityGameContractError::EqualityMismatch.into());
        }

        Ok(())
    }

    pub fn validate_same_resource(lt: Pubkey, rt: Pubkey) -> Result<(), ProgramError> {
        if lt == rt {
            return Err(MetalityGameContractError::SameUserJoining.into());
        }

        Ok(())
    }

    pub fn validate_bool(lt: bool, rt: bool) -> Result<(), ProgramError> {
        if lt != rt {
            return Err(MetalityGameContractError::EqualityMismatch.into());
        }

        Ok(())
    }

    pub fn validate_winner(
        state: MetalityGameContractState,
        winner: Pubkey,
    ) -> Result<(), ProgramError> {
        if state.user_a != winner && state.user_b != winner {
            return Err(MetalityGameContractError::InvalidWinner.into());
        }

        Ok(())
    }

    pub fn validate_state_account(
        game_state_account: &AccountInfo,
        program_id: Pubkey,
    ) -> Result<(), ProgramError> {
        if *game_state_account.owner != program_id || game_state_account.data_is_empty() {
            return Err(MetalityGameContractError::InvalidStateAccount.into());
        }

        Ok(())
    }
}
