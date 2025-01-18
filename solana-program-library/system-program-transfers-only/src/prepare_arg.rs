use crate::{
    instructions::{self, parser::Instruction},
    pb::sf::solana::block_meta::v1::SystemPrograpTransferOnlyArg,
};
use instructions::parser::parse_instruction;

pub fn prepare_arg(instruction_data: Vec<u8>, tx_id: String) -> SystemPrograpTransferOnlyArg {
    let mut arg: SystemPrograpTransferOnlyArg = SystemPrograpTransferOnlyArg::default();
    let instruction: Instruction = parse_instruction(instruction_data);

    arg.instruction_type = instruction.instructionType;

    match arg.instruction_type.as_str() {
        "Transfer" => {
            arg.transfer = Some(instruction.transfer.to_proto_struct());
        }
        "TransferWithSeed" => {
            arg.transfer_with_seed = Some(instruction.transferWithSeed.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
