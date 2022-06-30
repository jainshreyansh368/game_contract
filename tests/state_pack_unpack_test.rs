#[cfg(test)]
mod tests {
    use metality_game_contract::state::{MetalityGameContractState, MetalityGameProgramDataState};
    use solana_program::{program_pack::Pack, pubkey::Pubkey};

    #[test]
    fn game_state_pack_unpack_test() {
        let game_state = MetalityGameContractState {
            is_initialized: true,
            user_a: Pubkey::new(&[1; 32]),
            a_nft_ata: Pubkey::new(&[2; 32]),
            a_nft_mint: Pubkey::new(&[3; 32]),
            user_b: Pubkey::new(&[4; 32]),
            user_b_joined: true,
            b_nft_ata: Pubkey::new(&[5; 32]),
            b_nft_mint: Pubkey::new(&[6; 32]),
            pda_account: Pubkey::new(&[7; 32]),
            expired: false,
            game_started: true,
        };

        let mut packed = vec![0; MetalityGameContractState::get_packed_len()];

        MetalityGameContractState::pack(game_state, &mut packed).unwrap();

        let unpacked_data = MetalityGameContractState::unpack(&packed).unwrap();

        assert_eq!(game_state, unpacked_data);
    }

    #[test]
    fn program_data_state_pack_unpack_test() {
        let program_data_state = MetalityGameProgramDataState {
            is_initialized: true,
            index: 1,
        };

        let mut packed = vec![0; MetalityGameProgramDataState::get_packed_len()];

        MetalityGameProgramDataState::pack(program_data_state, &mut packed).unwrap();

        let unpacked_data = MetalityGameProgramDataState::unpack(&packed).unwrap();

        assert_eq!(program_data_state, unpacked_data);
    }
}
