//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use bytemuck::Pod;
use bytemuck::Zeroable;

unsafe impl Zeroable for OracleSetup {}
unsafe impl Pod for OracleSetup {}

#[repr(u8)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OracleSetup {
    None,
    PythLegacy,
    SwitchboardLegacy,
    PythPushOracle,
    SwitchboardPull,
}
