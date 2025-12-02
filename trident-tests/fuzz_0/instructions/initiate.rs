use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("6eksgdCnSjUaGQWZ6iYvauv1qzvYPF33RTGTM1ZuyENx")]
#[discriminator([5u8, 63u8, 123u8, 113u8, 153u8, 75u8, 148u8, 14u8])]
pub struct InitiateInstruction {
    pub accounts: InitiateInstructionAccounts,
    pub data: InitiateInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitiateInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitiateInstructionAccounts {
    #[account(mut)]
    pub swap_account: TridentAccount,

    #[account(mut, signer)]
    pub funder: TridentAccount,

    #[account(mut, signer)]
    pub rent_sponsor: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitiateInstructionData {
    pub redeemer: TridentPubkey,

    pub refundee: TridentPubkey,

    pub secret_hash: [u8; 32],

    pub swap_amount: u64,

    pub timelock: u64,

    pub destination_data: Option<Vec<u8>>,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitiateInstruction {
    type IxAccounts = FuzzAccounts;
}
