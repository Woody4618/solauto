//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::SolautoRebalanceType;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RebalanceData {
    pub rebalance_type: SolautoRebalanceType,
    pub padding1: [u8; 7],
    pub price_slippage_bps: u16,
    pub target_liq_utilization_rate_bps: u16,
    pub padding2: [u8; 4],
    pub flash_loan_amount: u64,
    pub padding: [u8; 32],
}