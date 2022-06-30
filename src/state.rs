use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetalityGameContractState {
    pub is_initialized: bool,
    pub user_a: Pubkey,
    pub a_nft_ata: Pubkey,
    pub a_nft_mint: Pubkey,
    pub user_b: Pubkey,
    pub user_b_joined: bool,
    pub b_nft_ata: Pubkey,
    pub b_nft_mint: Pubkey,
    pub pda_account: Pubkey,
    pub expired: bool,
    pub game_started: bool,
}

impl Sealed for MetalityGameContractState {}

impl IsInitialized for MetalityGameContractState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for MetalityGameContractState {
    const LEN: usize = 228;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, MetalityGameContractState::LEN];

        let (
            is_initialized,
            user_a,
            a_nft_ata,
            a_nft_mint,
            user_b,
            user_b_joined,
            b_nft_ata,
            b_nft_mint,
            pda_account,
            expired,
            game_started,
        ) = array_refs![src, 1, 32, 32, 32, 32, 1, 32, 32, 32, 1, 1];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let expired = match expired {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let user_b_joined = match user_b_joined {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let game_started = match game_started {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(MetalityGameContractState {
            is_initialized,
            user_a: Pubkey::new_from_array(*user_a),
            a_nft_ata: Pubkey::new_from_array(*a_nft_ata),
            a_nft_mint: Pubkey::new_from_array(*a_nft_mint),
            user_b: Pubkey::new_from_array(*user_b),
            user_b_joined,
            b_nft_ata: Pubkey::new_from_array(*b_nft_ata),
            b_nft_mint: Pubkey::new_from_array(*b_nft_mint),
            pda_account: Pubkey::new_from_array(*pda_account),
            expired,
            game_started,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dest = array_mut_ref![dst, 0, MetalityGameContractState::LEN];

        let (
            is_initialized_dest,
            user_a_dest,
            a_nft_ata_dest,
            a_nft_mint_dest,
            user_b_dest,
            user_b_joined_dest,
            b_nft_ata_dest,
            b_nft_mint_dest,
            pda_account_dest,
            expired_dest,
            game_started_dest,
        ) = mut_array_refs![dest, 1, 32, 32, 32, 32, 1, 32, 32, 32, 1, 1];

        let MetalityGameContractState {
            is_initialized,
            user_a,
            a_nft_ata,
            a_nft_mint,
            user_b,
            user_b_joined,
            b_nft_ata,
            b_nft_mint,
            pda_account,
            expired,
            game_started,
        } = self;

        is_initialized_dest[0] = *is_initialized as u8;
        user_a_dest.copy_from_slice(user_a.as_ref());
        a_nft_ata_dest.copy_from_slice(a_nft_ata.as_ref());
        a_nft_mint_dest.copy_from_slice(a_nft_mint.as_ref());
        user_b_dest.copy_from_slice(user_b.as_ref());
        user_b_joined_dest[0] = *user_b_joined as u8;
        b_nft_ata_dest.copy_from_slice(b_nft_ata.as_ref());
        b_nft_mint_dest.copy_from_slice(b_nft_mint.as_ref());
        pda_account_dest.copy_from_slice(pda_account.as_ref());
        expired_dest[0] = *expired as u8;
        game_started_dest[0] = *game_started as u8;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetalityGameProgramDataState {
    pub is_initialized: bool,
    pub index: u64,
}

impl Sealed for MetalityGameProgramDataState {}

impl IsInitialized for MetalityGameProgramDataState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for MetalityGameProgramDataState {
    const LEN: usize = 9;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, MetalityGameProgramDataState::LEN];

        let (is_initialized, index) = array_refs![src, 1, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(MetalityGameProgramDataState {
            is_initialized,
            index: u64::from_le_bytes(*index),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dest = array_mut_ref![dst, 0, MetalityGameProgramDataState::LEN];

        let (is_initialized_dest, index_dest) = mut_array_refs![dest, 1, 8];

        let MetalityGameProgramDataState {
            is_initialized,
            index,
        } = self;

        is_initialized_dest[0] = *is_initialized as u8;
        *index_dest = index.to_le_bytes();
    }
}
