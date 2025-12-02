use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("6eksgdCnSjUaGQWZ6iYvauv1qzvYPF33RTGTM1ZuyENx")]
#[discriminator([2u8, 96u8, 183u8, 251u8, 63u8, 208u8, 46u8, 46u8])]
pub struct RefundInstruction {
    pub accounts: RefundInstructionAccounts,
    pub data: RefundInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(RefundInstructionData)]
#[storage(FuzzAccounts)]
pub struct RefundInstructionAccounts {
    #[account(mut)]
    pub swap_account: TridentAccount,

    #[account(mut)]
    pub refundee: TridentAccount,

    #[account(mut)]
    pub rent_sponsor: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct RefundInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for RefundInstruction {
    type IxAccounts = FuzzAccounts;
}
