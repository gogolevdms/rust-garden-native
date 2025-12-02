use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Initiated {
    pub redeemer: TridentPubkey,

    pub refundee: TridentPubkey,

    pub secret_hash: [u8; 32],

    pub swap_amount: u64,

    pub timelock: u64,

    pub destination_data: Option<Vec<u8>>,

    pub funder: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InstantRefunded {
    pub redeemer: TridentPubkey,

    pub refundee: TridentPubkey,

    pub secret_hash: [u8; 32],

    pub swap_amount: u64,

    pub timelock: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Redeemed {
    pub redeemer: TridentPubkey,

    pub refundee: TridentPubkey,

    pub secret: [u8; 32],

    pub swap_amount: u64,

    pub timelock: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Refunded {
    pub redeemer: TridentPubkey,

    pub refundee: TridentPubkey,

    pub secret_hash: [u8; 32],

    pub swap_amount: u64,

    pub timelock: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SwapAccount {
    pub expiry_slot: u64,

    pub bump: u8,

    pub redeemer: TridentPubkey,

    pub refundee: TridentPubkey,

    pub secret_hash: [u8; 32],

    pub swap_amount: u64,

    pub rent_sponsor: TridentPubkey,

    pub timelock: u64,
}
