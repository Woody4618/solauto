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

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WrappedI80F48 {
    pub value: [u8; 16],
}