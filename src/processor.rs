use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token;

use crate::instruction::MetalityGameContractInstruction;
use crate::state::{MetalityGameContractState, MetalityGameProgramDataState};
use crate::validations::Validator;

pub struct Processor;

impl Processor {
    pub fn unpack_and_process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        match MetalityGameContractInstruction::unpack_instruction_data(instruction_data)? {
            MetalityGameContractInstruction::InitializeGameProgramData => {
                msg!("Instruction: InitializeGameProgramData (Admin)");
                Self::process_initialize_game_program_data(accounts, program_id)?;
            }

            MetalityGameContractInstruction::InitializeGame => {
                msg!("Instruction: InitializeGame");
                Self::process_initialize_game(accounts, program_id)?;
            }

            MetalityGameContractInstruction::CancelGame => {
                msg!("Instruction: CancelGame");
                Self::process_cancel_game(accounts, program_id)?;
            }

            MetalityGameContractInstruction::JoinGame => {
                msg!("Instruction: JoinGame");
                Self::process_join_game(accounts, program_id)?;
            }

            MetalityGameContractInstruction::TransferReward => {
                msg!("Instruction: TransferReward (Admin)");
                Self::process_transfer_reward(accounts, program_id)?;
            }

            MetalityGameContractInstruction::DrawOrCancelGame => {
                msg!("Instruction: DrawOrCancelGame (Admin)");
                Self::process_draw_or_cancel_game(accounts, program_id)?;
            }

            MetalityGameContractInstruction::SetGameStarted => {
                msg!("Instruction: SetGameStarted (Admin)");
                Self::process_set_game_started(accounts, program_id)?;
            }

            MetalityGameContractInstruction::UserBExitGame => {
                msg!("Instruction: UserBExitGame");
                Self::process_user_b_exit_game(accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_initialize_game_program_data(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let admin_account = next_account_info(account_info_iter)?;

        let game_program_data_account = next_account_info(account_info_iter)?;

        let system_program_account = next_account_info(account_info_iter)?;

        Validator::validate_admin(admin_account)?;

        let create_game_program_data_state_ix = system_instruction::create_account_with_seed(
            admin_account.key,
            game_program_data_account.key,
            admin_account.key,
            "Game Contract Main",
            Rent::default().minimum_balance(MetalityGameProgramDataState::LEN),
            MetalityGameProgramDataState::LEN as u64,
            program_id,
        );

        invoke(
            &create_game_program_data_state_ix,
            &[
                admin_account.clone(),
                game_program_data_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        let mut game_program_data_unpacked = MetalityGameProgramDataState::unpack_unchecked(
            &game_program_data_account.try_borrow_data()?,
        )?;

        game_program_data_unpacked.index = 0;
        game_program_data_unpacked.is_initialized = true;

        MetalityGameProgramDataState::pack(
            game_program_data_unpacked,
            &mut game_program_data_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    pub fn process_initialize_game(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user_a = next_account_info(account_info_iter)?;

        let nft_ata = next_account_info(account_info_iter)?;

        let nft_mint = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let game_program_data_account = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let system_program_account = next_account_info(account_info_iter)?;

        let mut game_program_data_unpacked = MetalityGameProgramDataState::unpack_unchecked(
            &game_program_data_account.try_borrow_data()?,
        )?;

        msg!("Index: {:?}", game_program_data_unpacked.index);

        let (pda, _bump_seeds) = Pubkey::find_program_address(
            &[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
            ],
            program_id,
        );

        let seed = format!("Metality Game State {}", game_program_data_unpacked.index);

        Validator::validate_is_signer(user_a)?;
        Validator::validate_token_owner(nft_ata, user_a)?;
        Validator::validate_token_ata(nft_ata, nft_mint)?;
        Validator::validate_equality(*pda_account.key, pda)?;

        let create_game_state_with_seed_ix = system_instruction::create_account_with_seed(
            user_a.key,
            game_state_account.key,
            user_a.key,
            &seed,
            Rent::default().minimum_balance(MetalityGameContractState::LEN),
            MetalityGameContractState::LEN as u64,
            program_id,
        );

        invoke(
            &create_game_state_with_seed_ix,
            &[
                user_a.clone(),
                game_state_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        let mut game_state_unpacked =
            MetalityGameContractState::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        let set_authority_pda_ins = spl_token::instruction::set_authority(
            &spl_token::ID,
            nft_ata.key,
            Some(pda_account.key),
            spl_token::instruction::AuthorityType::AccountOwner,
            user_a.key,
            &[user_a.key],
        )?;

        invoke(
            &set_authority_pda_ins,
            &[
                nft_ata.clone(),
                user_a.clone(),
                token_program_account.clone(),
            ],
        )?;

        game_state_unpacked.is_initialized = true;
        game_state_unpacked.user_a = *user_a.key;
        game_state_unpacked.a_nft_ata = *nft_ata.key;
        game_state_unpacked.a_nft_mint = *nft_mint.key;
        game_state_unpacked.pda_account = *pda_account.key;
        game_state_unpacked.expired = false;
        game_state_unpacked.game_started = false;

        MetalityGameContractState::pack(
            game_state_unpacked,
            &mut game_state_account.try_borrow_mut_data()?,
        )?;

        game_program_data_unpacked.index += 1;

        MetalityGameProgramDataState::pack(
            game_program_data_unpacked,
            &mut game_program_data_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    pub fn process_cancel_game(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user_a = next_account_info(account_info_iter)?;

        let a_nft_ata = next_account_info(account_info_iter)?;

        let a_nft_mint = next_account_info(account_info_iter)?;

        let user_b = next_account_info(account_info_iter)?;

        let b_nft_ata = next_account_info(account_info_iter)?;

        let b_nft_mint = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let seeds = [
            "metality_game_contract".as_bytes(),
            game_state_account.key.as_ref(),
        ];

        let (pda, bump_seeds) = Pubkey::find_program_address(&seeds, program_id);

        let mut game_state_unpacked =
            MetalityGameContractState::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        Validator::validate_is_signer(user_a)?;
        Validator::validate_state_account(game_state_account, *program_id)?;
        Validator::validate_bool(game_state_unpacked.is_initialized, true)?;
        Validator::validate_bool(game_state_unpacked.expired, false)?;
        Validator::validate_bool(game_state_unpacked.game_started, false)?;
        Validator::validate_equality(game_state_unpacked.user_a, *user_a.key)?;
        Validator::validate_equality(*pda_account.key, pda)?;
        Validator::validate_equality(pda, game_state_unpacked.pda_account)?;
        Validator::validate_equality(*a_nft_ata.key, game_state_unpacked.a_nft_ata)?;
        Validator::validate_equality(*a_nft_mint.key, game_state_unpacked.a_nft_mint)?;
        Validator::validate_token_ata(a_nft_ata, a_nft_mint)?;
        Validator::validate_token_owner(a_nft_ata, pda_account)?;

        let mut set_authority_user_ins = spl_token::instruction::set_authority(
            &spl_token::ID,
            a_nft_ata.key,
            Some(user_a.key),
            spl_token::instruction::AuthorityType::AccountOwner,
            pda_account.key,
            &[],
        )?;

        invoke_signed(
            &set_authority_user_ins,
            &[
                a_nft_ata.clone(),
                pda_account.clone(),
                token_program_account.clone(),
            ],
            &[&[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
                &[bump_seeds],
            ]],
        )?;

        if game_state_unpacked.user_b_joined {
            Validator::validate_bool(game_state_unpacked.user_b_joined, true)?;
            Validator::validate_equality(game_state_unpacked.user_b, *user_b.key)?;
            Validator::validate_equality(*b_nft_ata.key, game_state_unpacked.b_nft_ata)?;
            Validator::validate_equality(*b_nft_mint.key, game_state_unpacked.b_nft_mint)?;
            Validator::validate_token_ata(b_nft_ata, b_nft_mint)?;
            Validator::validate_token_owner(b_nft_ata, pda_account)?;

            set_authority_user_ins = spl_token::instruction::set_authority(
                &spl_token::ID,
                b_nft_ata.key,
                Some(user_b.key),
                spl_token::instruction::AuthorityType::AccountOwner,
                pda_account.key,
                &[],
            )?;

            invoke_signed(
                &set_authority_user_ins,
                &[
                    b_nft_ata.clone(),
                    pda_account.clone(),
                    token_program_account.clone(),
                ],
                &[&[
                    "metality_game_contract".as_bytes(),
                    game_state_account.key.as_ref(),
                    &[bump_seeds],
                ]],
            )?;
        }

        game_state_unpacked.expired = true;

        MetalityGameContractState::pack(
            game_state_unpacked,
            &mut game_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    pub fn process_join_game(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user_b = next_account_info(account_info_iter)?;

        let nft_ata = next_account_info(account_info_iter)?;

        let nft_mint = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let (pda, _bump_seeds) = Pubkey::find_program_address(
            &[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
            ],
            program_id,
        );

        let mut game_state_unpacked =
            MetalityGameContractState::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        Validator::validate_is_signer(user_b)?;
        Validator::validate_state_account(game_state_account, *program_id)?;
        Validator::validate_bool(game_state_unpacked.is_initialized, true)?;
        Validator::validate_bool(game_state_unpacked.user_b_joined, false)?;
        Validator::validate_token_owner(nft_ata, user_b)?;
        Validator::validate_token_ata(nft_ata, nft_mint)?;
        Validator::validate_equality(*pda_account.key, pda)?;
        Validator::validate_equality(*pda_account.key, game_state_unpacked.pda_account)?;
        Validator::validate_same_resource(game_state_unpacked.user_a, *user_b.key)?;
        Validator::validate_same_resource(game_state_unpacked.a_nft_ata, *nft_ata.key)?;
        Validator::validate_same_resource(game_state_unpacked.a_nft_mint, *nft_mint.key)?;

        let set_authority_pda_ins = spl_token::instruction::set_authority(
            &spl_token::ID,
            nft_ata.key,
            Some(pda_account.key),
            spl_token::instruction::AuthorityType::AccountOwner,
            user_b.key,
            &[user_b.key],
        )?;

        invoke(
            &set_authority_pda_ins,
            &[
                nft_ata.clone(),
                user_b.clone(),
                token_program_account.clone(),
            ],
        )?;

        game_state_unpacked.user_b = *user_b.key;
        game_state_unpacked.b_nft_ata = *nft_ata.key;
        game_state_unpacked.b_nft_mint = *nft_mint.key;
        game_state_unpacked.user_b_joined = true;

        MetalityGameContractState::pack(
            game_state_unpacked,
            &mut game_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    pub fn process_transfer_reward(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let admin = next_account_info(account_info_iter)?;

        let winner = next_account_info(account_info_iter)?;

        let loser = next_account_info(account_info_iter)?;

        let won_nft = next_account_info(account_info_iter)?;

        let won_nft_mint = next_account_info(account_info_iter)?;

        let owned_nft = next_account_info(account_info_iter)?;

        let owned_nft_mint = next_account_info(account_info_iter)?;

        let winner_won_nft_ata = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let system_program_account = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let rent_sysvar_account = next_account_info(account_info_iter)?;

        let associated_token_account_program = next_account_info(account_info_iter)?;

        let (pda, bump_seeds) = Pubkey::find_program_address(
            &[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
            ],
            program_id,
        );

        let mut game_state_unpacked =
            MetalityGameContractState::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        let winner_won_nft_ata_pubkey = get_associated_token_address(winner.key, won_nft_mint.key);

        Validator::validate_admin(admin)?;
        // Validator::validate_is_signer(winner)?;
        Validator::validate_state_account(game_state_account, *program_id)?;
        Validator::validate_winner(game_state_unpacked, *winner.key)?;
        Validator::validate_bool(game_state_unpacked.is_initialized, true)?;
        Validator::validate_bool(game_state_unpacked.user_b_joined, true)?;
        Validator::validate_bool(game_state_unpacked.expired, false)?;
        Validator::validate_bool(game_state_unpacked.game_started, true)?;
        Validator::validate_token_owner(won_nft, pda_account)?;
        Validator::validate_token_owner(owned_nft, pda_account)?;
        Validator::validate_token_ata(won_nft, won_nft_mint)?;
        Validator::validate_token_ata(owned_nft, owned_nft_mint)?;
        Validator::validate_equality(*pda_account.key, pda)?;
        Validator::validate_equality(*pda_account.key, game_state_unpacked.pda_account)?;
        Validator::validate_equality(winner_won_nft_ata_pubkey, *winner_won_nft_ata.key)?;

        if winner_won_nft_ata.data_is_empty() {
            invoke(
                &create_associated_token_account(admin.key, winner.key, won_nft_mint.key),
                &[
                    admin.clone(),
                    winner_won_nft_ata.clone(),
                    winner.clone(),
                    won_nft_mint.clone(),
                    system_program_account.clone(),
                    token_program_account.clone(),
                    rent_sysvar_account.clone(),
                    associated_token_account_program.clone(),
                ],
            )?;
        }

        invoke_signed(
            &spl_token::instruction::set_authority(
                &spl_token::ID,
                owned_nft.key,
                Some(winner.key),
                spl_token::instruction::AuthorityType::AccountOwner,
                pda_account.key,
                &[],
            )?,
            &[
                owned_nft.clone(),
                pda_account.clone(),
                token_program_account.clone(),
            ],
            &[&[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
                &[bump_seeds],
            ]],
        )?;

        invoke_signed(
            &spl_token::instruction::transfer(
                &spl_token::ID,
                won_nft.key,
                winner_won_nft_ata.key,
                pda_account.key,
                &[],
                1,
            )?,
            &[
                won_nft.clone(),
                winner_won_nft_ata.clone(),
                pda_account.clone(),
            ],
            &[&[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
                &[bump_seeds],
            ]],
        )?;

        invoke_signed(
            &spl_token::instruction::set_authority(
                &spl_token::ID,
                won_nft.key,
                Some(loser.key),
                spl_token::instruction::AuthorityType::AccountOwner,
                pda_account.key,
                &[],
            )?,
            &[
                won_nft.clone(),
                pda_account.clone(),
                token_program_account.clone(),
            ],
            &[&[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
                &[bump_seeds],
            ]],
        )?;

        game_state_unpacked.expired = true;

        MetalityGameContractState::pack(
            game_state_unpacked,
            &mut game_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    pub fn process_draw_or_cancel_game(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let admin = next_account_info(account_info_iter)?;

        let user_a = next_account_info(account_info_iter)?;

        let user_b = next_account_info(account_info_iter)?;

        let a_nft_ata = next_account_info(account_info_iter)?;

        let a_nft_mint = next_account_info(account_info_iter)?;

        let b_nft_ata = next_account_info(account_info_iter)?;

        let b_nft_mint = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let (pda, bump_seeds) = Pubkey::find_program_address(
            &[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
            ],
            program_id,
        );

        let mut game_state_unpacked =
            MetalityGameContractState::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        Validator::validate_admin(admin)?;
        Validator::validate_state_account(game_state_account, *program_id)?;
        Validator::validate_bool(game_state_unpacked.is_initialized, true)?;
        Validator::validate_bool(game_state_unpacked.expired, false)?;
        Validator::validate_token_owner(a_nft_ata, pda_account)?;
        Validator::validate_token_ata(a_nft_ata, a_nft_mint)?;
        Validator::validate_equality(*user_a.key, game_state_unpacked.user_a)?;
        Validator::validate_equality(*a_nft_ata.key, game_state_unpacked.a_nft_ata)?;
        Validator::validate_equality(*a_nft_mint.key, game_state_unpacked.a_nft_mint)?;
        Validator::validate_equality(*pda_account.key, pda)?;
        Validator::validate_equality(*pda_account.key, game_state_unpacked.pda_account)?;

        let mut set_authority_user_ins = spl_token::instruction::set_authority(
            &spl_token::ID,
            a_nft_ata.key,
            Some(user_a.key),
            spl_token::instruction::AuthorityType::AccountOwner,
            pda_account.key,
            &[],
        )?;

        invoke_signed(
            &set_authority_user_ins,
            &[
                a_nft_ata.clone(),
                pda_account.clone(),
                token_program_account.clone(),
            ],
            &[&[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
                &[bump_seeds],
            ]],
        )?;

        if game_state_unpacked.user_b_joined {
            Validator::validate_bool(game_state_unpacked.user_b_joined, true)?;
            Validator::validate_token_owner(b_nft_ata, pda_account)?;
            Validator::validate_token_ata(b_nft_ata, b_nft_mint)?;
            Validator::validate_equality(*user_b.key, game_state_unpacked.user_b)?;
            Validator::validate_equality(*b_nft_ata.key, game_state_unpacked.b_nft_ata)?;
            Validator::validate_equality(*b_nft_mint.key, game_state_unpacked.b_nft_mint)?;

            set_authority_user_ins = spl_token::instruction::set_authority(
                &spl_token::ID,
                b_nft_ata.key,
                Some(user_b.key),
                spl_token::instruction::AuthorityType::AccountOwner,
                pda_account.key,
                &[],
            )?;

            invoke_signed(
                &set_authority_user_ins,
                &[
                    b_nft_ata.clone(),
                    pda_account.clone(),
                    token_program_account.clone(),
                ],
                &[&[
                    "metality_game_contract".as_bytes(),
                    game_state_account.key.as_ref(),
                    &[bump_seeds],
                ]],
            )?;
        }

        game_state_unpacked.expired = true;

        MetalityGameContractState::pack(
            game_state_unpacked,
            &mut game_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    pub fn process_set_game_started(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let admin = next_account_info(account_info_iter)?;

        let user_a = next_account_info(account_info_iter)?;

        let user_b = next_account_info(account_info_iter)?;

        let a_nft_ata = next_account_info(account_info_iter)?;

        let a_nft_mint = next_account_info(account_info_iter)?;

        let b_nft_ata = next_account_info(account_info_iter)?;

        let b_nft_mint = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let (pda, _bump_seeds) = Pubkey::find_program_address(
            &[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
            ],
            program_id,
        );

        let mut game_state_unpacked =
            MetalityGameContractState::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        Validator::validate_admin(admin)?;
        Validator::validate_state_account(game_state_account, *program_id)?;
        Validator::validate_bool(game_state_unpacked.is_initialized, true)?;
        Validator::validate_bool(game_state_unpacked.user_b_joined, true)?;
        Validator::validate_bool(game_state_unpacked.expired, false)?;
        Validator::validate_bool(game_state_unpacked.game_started, false)?;
        Validator::validate_equality(*user_a.key, game_state_unpacked.user_a)?;
        Validator::validate_equality(*user_b.key, game_state_unpacked.user_b)?;
        Validator::validate_equality(*a_nft_ata.key, game_state_unpacked.a_nft_ata)?;
        Validator::validate_equality(*a_nft_mint.key, game_state_unpacked.a_nft_mint)?;
        Validator::validate_equality(*b_nft_ata.key, game_state_unpacked.b_nft_ata)?;
        Validator::validate_equality(*b_nft_mint.key, game_state_unpacked.b_nft_mint)?;
        Validator::validate_equality(*pda_account.key, pda)?;
        Validator::validate_token_owner(a_nft_ata, pda_account)?;
        Validator::validate_token_owner(b_nft_ata, pda_account)?;
        Validator::validate_token_ata(a_nft_ata, a_nft_mint)?;
        Validator::validate_token_ata(b_nft_ata, b_nft_mint)?;

        game_state_unpacked.game_started = true;

        MetalityGameContractState::pack(
            game_state_unpacked,
            &mut game_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    pub fn process_user_b_exit_game(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user_b = next_account_info(account_info_iter)?;

        let b_nft_ata = next_account_info(account_info_iter)?;

        let b_nft_mint = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let token_program_account = next_account_info(account_info_iter)?;

        let (pda, bump_seeds) = Pubkey::find_program_address(
            &[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
            ],
            program_id,
        );

        let mut game_state_unpacked =
            MetalityGameContractState::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        Validator::validate_is_signer(user_b)?;
        Validator::validate_state_account(game_state_account, *program_id)?;
        Validator::validate_bool(game_state_unpacked.is_initialized, true)?;
        Validator::validate_bool(game_state_unpacked.user_b_joined, true)?;
        Validator::validate_bool(game_state_unpacked.expired, false)?;
        Validator::validate_bool(game_state_unpacked.game_started, false)?;
        Validator::validate_equality(*user_b.key, game_state_unpacked.user_b)?;
        Validator::validate_equality(*b_nft_ata.key, game_state_unpacked.b_nft_ata)?;
        Validator::validate_equality(*b_nft_mint.key, game_state_unpacked.b_nft_mint)?;
        Validator::validate_equality(*pda_account.key, pda)?;
        Validator::validate_token_owner(b_nft_ata, pda_account)?;
        Validator::validate_token_ata(b_nft_ata, b_nft_mint)?;

        let set_authority_user_ins = spl_token::instruction::set_authority(
            &spl_token::ID,
            b_nft_ata.key,
            Some(user_b.key),
            spl_token::instruction::AuthorityType::AccountOwner,
            pda_account.key,
            &[],
        )?;

        invoke_signed(
            &set_authority_user_ins,
            &[
                b_nft_ata.clone(),
                pda_account.clone(),
                token_program_account.clone(),
            ],
            &[&[
                "metality_game_contract".as_bytes(),
                game_state_account.key.as_ref(),
                &[bump_seeds],
            ]],
        )?;

        game_state_unpacked.user_b_joined = false;
        game_state_unpacked.user_b = Pubkey::default();
        game_state_unpacked.b_nft_ata = Pubkey::default();
        game_state_unpacked.b_nft_mint = Pubkey::default();

        MetalityGameContractState::pack(
            game_state_unpacked,
            &mut game_state_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }
}
