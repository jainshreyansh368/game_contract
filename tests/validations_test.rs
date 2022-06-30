use solana_sdk::account_info::AccountInfo;
use solana_sdk::program_pack::Pack;
use solana_sdk::{account::Account, account_info::IntoAccountInfo, pubkey::Pubkey};
use spl_token::state::{Account as TokenAccount, AccountState, Mint};

pub fn get_token_ata<'a>(
    mint_pubkey: u8,
    owner_pubkey: u8,
    pubkey: &'a Pubkey,
    token_account: &'a mut Account,
) -> AccountInfo<'a> {
    let mut default_account = TokenAccount::default();

    default_account.state = AccountState::Initialized;
    default_account.mint = Pubkey::new_from_array([mint_pubkey; 32]);
    default_account.owner = Pubkey::new_from_array([owner_pubkey; 32]);
    default_account.amount = 1;

    let token_account_info = (pubkey, false, token_account).into_account_info();

    TokenAccount::pack(
        default_account,
        &mut token_account_info.try_borrow_mut_data().unwrap(),
    )
    .unwrap();

    return token_account_info;
}

pub fn get_mint_account<'a>(pubkey: &'a Pubkey, mint_account: &'a mut Account) -> AccountInfo<'a> {
    let mut default_account = Mint::default();

    default_account.is_initialized = true;
    default_account.decimals = 0;
    default_account.supply = 1;

    let mint_account_info = (pubkey, false, mint_account).into_account_info();

    Mint::pack(
        default_account,
        &mut mint_account_info.try_borrow_mut_data().unwrap(),
    )
    .unwrap();

    return mint_account_info;
}

pub fn get_account(size: usize, owner: Pubkey) -> Account {
    let account = Account {
        lamports: u32::MAX as u64,
        data: vec![0u8; size],
        owner,
        executable: false,
        rent_epoch: 0,
    };

    return account;
}

pub mod admin {
    solana_sdk::declare_id!("3KBgdH5xuVWKVB85L3SaRAiHXhDb77yd9qc6rxpNL2hr");
}

#[cfg(test)]
mod tests {
    use super::*;
    use metality_game_contract::validations::Validator;

    #[test]
    fn success_validate_token_ata_test() {
        let mut mint_account = get_account(Mint::LEN, spl_token::id());
        let mint_pubkey = Pubkey::new_from_array([11; 32]);
        let mint_account_info = get_mint_account(&mint_pubkey, &mut mint_account);

        let mut token_account = get_account(TokenAccount::LEN, spl_token::id());
        let token_ata_pubkey = Pubkey::new_from_array([1; 32]);
        let token_account_info = get_token_ata(11, 22, &token_ata_pubkey, &mut token_account);

        assert_eq!(
            Validator::validate_token_ata(&token_account_info, &mint_account_info).unwrap(),
            ()
        )
    }

    #[test]
    #[should_panic]
    fn failure_validate_token_ata_test() {
        let mut mint_account = get_account(Mint::LEN, spl_token::id());
        let mint_pubkey = Pubkey::new_from_array([10; 32]);
        let mint_account_info = get_mint_account(&mint_pubkey, &mut mint_account);

        let mut token_account = get_account(TokenAccount::LEN, spl_token::id());
        let token_ata_pubkey = Pubkey::new_from_array([1; 32]);
        let token_account_info = get_token_ata(11, 22, &token_ata_pubkey, &mut token_account);

        assert_eq!(
            Validator::validate_token_ata(&token_account_info, &mint_account_info).unwrap(),
            ()
        )
    }

    #[test]
    fn success_validate_token_owner_test() {
        let mut user_account = get_account(0, solana_sdk::system_program::id());
        let account_pubkey = Pubkey::new_from_array([10; 32]);
        let user_account_info = (&account_pubkey, false, &mut user_account).into_account_info();

        let mut token_account = get_account(TokenAccount::LEN, spl_token::id());
        let token_ata_pubkey = Pubkey::new_from_array([1; 32]);
        let token_account_info = get_token_ata(11, 10, &token_ata_pubkey, &mut token_account);

        Validator::validate_token_owner(&token_account_info, &user_account_info).unwrap();
    }

    #[test]
    #[should_panic]
    fn failure_validate_token_owner_test() {
        let mut user_account = get_account(0, solana_sdk::system_program::id());
        let account_pubkey = Pubkey::new_from_array([10; 32]);
        let user_account_info = (&account_pubkey, false, &mut user_account).into_account_info();

        let mut token_account = get_account(TokenAccount::LEN, spl_token::id());
        let token_ata_pubkey = Pubkey::new_from_array([1; 32]);
        let token_account_info = get_token_ata(11, 11, &token_ata_pubkey, &mut token_account);

        Validator::validate_token_owner(&token_account_info, &user_account_info).unwrap();
    }

    #[test]
    fn success_validate_is_signer_test() {
        let mut user_account = get_account(0, solana_sdk::system_program::id());
        let account_pubkey = Pubkey::new_from_array([10; 32]);
        let user_account_info = (&account_pubkey, true, &mut user_account).into_account_info();

        assert_eq!(
            Validator::validate_is_signer(&user_account_info).unwrap(),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn failure_validate_is_signer_test() {
        let mut user_account = get_account(0, solana_sdk::system_program::id());
        let account_pubkey = Pubkey::new_from_array([10; 32]);
        let user_account_info = (&account_pubkey, false, &mut user_account).into_account_info();

        Validator::validate_is_signer(&user_account_info).unwrap();
    }

    #[test]
    fn success_validate_admin_test() {
        let mut admin_account = get_account(0, solana_sdk::system_program::id());
        let admin_pubkey = admin::id();
        let admin_account_info = (&admin_pubkey, true, &mut admin_account).into_account_info();

        assert_eq!(Validator::validate_admin(&admin_account_info).unwrap(), ());
    }

    #[test]
    #[should_panic]
    fn failure_validate_admin_test() {
        let mut admin_account = get_account(0, solana_sdk::system_program::id());
        let admin_pubkey = admin::id();
        let admin_account_info = (&admin_pubkey, false, &mut admin_account).into_account_info();

        Validator::validate_admin(&admin_account_info).unwrap();
    }

    #[test]
    #[should_panic]
    fn failure_validate_admin_test_two() {
        let mut admin_account = get_account(0, solana_sdk::system_program::id());
        let admin_pubkey = Pubkey::new_from_array([10; 32]);
        let admin_account_info = (&admin_pubkey, true, &mut admin_account).into_account_info();

        Validator::validate_admin(&admin_account_info).unwrap();
    }

    #[test]
    fn success_validate_equality_test() {
        let one_pubkey = Pubkey::new_from_array([10; 32]);
        let two_pubkey = Pubkey::new_from_array([10; 32]);

        assert_eq!(
            Validator::validate_equality(one_pubkey, two_pubkey).unwrap(),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn failure_validate_equality_test() {
        let one_pubkey = Pubkey::new_from_array([10; 32]);
        let two_pubkey = Pubkey::new_from_array([11; 32]);

        Validator::validate_equality(one_pubkey, two_pubkey).unwrap();
    }

    #[test]
    fn success_validate_same_resource_test() {
        let one_pubkey = Pubkey::new_from_array([10; 32]);
        let two_pubkey = Pubkey::new_from_array([11; 32]);

        assert_eq!(
            Validator::validate_same_resource(one_pubkey, two_pubkey).unwrap(),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn failure_validate_same_resource() {
        let one_pubkey = Pubkey::new_from_array([10; 32]);
        let two_pubkey = Pubkey::new_from_array([10; 32]);

        Validator::validate_same_resource(one_pubkey, two_pubkey).unwrap();
    }

    #[test]
    fn success_validate_bool_test() {
        assert_eq!(Validator::validate_bool(true, true).unwrap(), ());
    }

    #[test]
    #[should_panic]
    fn failure_validate_bool_test() {
        assert_eq!(Validator::validate_bool(true, false).unwrap(), ());
    }

    #[test]
    fn success_validate_winner_test() {
        use metality_game_contract::state::MetalityGameContractState;

        let winner_pubkey = Pubkey::new_from_array([10; 32]);

        let game_state = MetalityGameContractState {
            is_initialized: true,
            user_a: Pubkey::new_from_array([10; 32]),
            a_nft_ata: Pubkey::new_from_array([1; 32]),
            a_nft_mint: Pubkey::new_from_array([2; 32]),
            user_b: Pubkey::new_from_array([20; 32]),
            user_b_joined: true,
            b_nft_ata: Pubkey::new_from_array([3; 32]),
            b_nft_mint: Pubkey::new_from_array([4; 32]),
            pda_account: Pubkey::new_from_array([99; 32]),
            expired: false,
            game_started: true,
        };

        assert_eq!(
            Validator::validate_winner(game_state, winner_pubkey).unwrap(),
            ()
        );
    }

    #[test]
    fn success_validate_winner_test_two() {
        use metality_game_contract::state::MetalityGameContractState;

        let winner_pubkey = Pubkey::new_from_array([20; 32]);

        let game_state = MetalityGameContractState {
            is_initialized: true,
            user_a: Pubkey::new_from_array([10; 32]),
            a_nft_ata: Pubkey::new_from_array([1; 32]),
            a_nft_mint: Pubkey::new_from_array([2; 32]),
            user_b: Pubkey::new_from_array([20; 32]),
            user_b_joined: true,
            b_nft_ata: Pubkey::new_from_array([3; 32]),
            b_nft_mint: Pubkey::new_from_array([4; 32]),
            pda_account: Pubkey::new_from_array([99; 32]),
            expired: false,
            game_started: true,
        };

        assert_eq!(
            Validator::validate_winner(game_state, winner_pubkey).unwrap(),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn failure_validate_winner_test() {
        use metality_game_contract::state::MetalityGameContractState;

        let winner_pubkey = Pubkey::new_from_array([33; 32]);

        let game_state = MetalityGameContractState {
            is_initialized: true,
            user_a: Pubkey::new_from_array([10; 32]),
            a_nft_ata: Pubkey::new_from_array([1; 32]),
            a_nft_mint: Pubkey::new_from_array([2; 32]),
            user_b: Pubkey::new_from_array([20; 32]),
            user_b_joined: true,
            b_nft_ata: Pubkey::new_from_array([3; 32]),
            b_nft_mint: Pubkey::new_from_array([4; 32]),
            pda_account: Pubkey::new_from_array([99; 32]),
            expired: false,
            game_started: true,
        };

        Validator::validate_winner(game_state, winner_pubkey).unwrap();
    }

    #[test]
    fn success_validate_state_account_test() {
        use metality_game_contract::state::MetalityGameContractState;

        let program_id = Pubkey::new_from_array([55; 32]);
        let mut state_account = get_account(MetalityGameContractState::LEN, program_id);
        let state_account_pubkey = Pubkey::new_from_array([2; 32]);
        let state_account_info =
            (&state_account_pubkey, false, &mut state_account).into_account_info();

        assert_eq!(
            Validator::validate_state_account(&state_account_info, program_id).unwrap(),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn failure_validate_state_account_test() {
        let program_id = Pubkey::new_from_array([55; 32]);
        let mut state_account = get_account(0, program_id);
        let state_account_pubkey = Pubkey::new_from_array([2; 32]);
        let state_account_info =
            (&state_account_pubkey, false, &mut state_account).into_account_info();

        assert_eq!(
            Validator::validate_state_account(&state_account_info, program_id).unwrap(),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn failure_validate_state_account_test_two() {
        use metality_game_contract::state::MetalityGameContractState;

        let program_id = Pubkey::new_from_array([55; 32]);
        let mut state_account = get_account(MetalityGameContractState::LEN, program_id);
        let state_account_pubkey = Pubkey::new_from_array([2; 32]);
        let state_account_info =
            (&state_account_pubkey, false, &mut state_account).into_account_info();

        assert_eq!(
            Validator::validate_state_account(
                &state_account_info,
                Pubkey::new_from_array([56; 32])
            )
            .unwrap(),
            ()
        );
    }
}
