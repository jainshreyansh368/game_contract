#[cfg(test)]
mod tests {
    use metality_game_contract::instruction::MetalityGameContractInstruction;

    #[test]
    fn instruction_data_unpack_test() {
        let mut packed_ins_data = [0u8];

        let mut unpacked_ins_data =
            MetalityGameContractInstruction::unpack_instruction_data(&packed_ins_data).unwrap();

        assert_eq!(
            unpacked_ins_data,
            MetalityGameContractInstruction::InitializeGameProgramData
        );

        packed_ins_data = [1u8];

        unpacked_ins_data =
            MetalityGameContractInstruction::unpack_instruction_data(&packed_ins_data).unwrap();

        assert_eq!(
            unpacked_ins_data,
            MetalityGameContractInstruction::InitializeGame
        );

        packed_ins_data = [2u8];

        unpacked_ins_data =
            MetalityGameContractInstruction::unpack_instruction_data(&packed_ins_data).unwrap();

        assert_eq!(
            unpacked_ins_data,
            MetalityGameContractInstruction::CancelGame
        );

        packed_ins_data = [3u8];

        unpacked_ins_data =
            MetalityGameContractInstruction::unpack_instruction_data(&packed_ins_data).unwrap();

        assert_eq!(unpacked_ins_data, MetalityGameContractInstruction::JoinGame);

        packed_ins_data = [4u8];

        unpacked_ins_data =
            MetalityGameContractInstruction::unpack_instruction_data(&packed_ins_data).unwrap();

        assert_eq!(
            unpacked_ins_data,
            MetalityGameContractInstruction::TransferReward
        );

        packed_ins_data = [5u8];

        unpacked_ins_data =
            MetalityGameContractInstruction::unpack_instruction_data(&packed_ins_data).unwrap();

        assert_eq!(
            unpacked_ins_data,
            MetalityGameContractInstruction::DrawOrCancelGame
        );

        packed_ins_data = [6u8];

        unpacked_ins_data =
            MetalityGameContractInstruction::unpack_instruction_data(&packed_ins_data).unwrap();

        assert_eq!(
            unpacked_ins_data,
            MetalityGameContractInstruction::SetGameStarted
        );
    }
}
