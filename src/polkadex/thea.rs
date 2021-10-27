//! Implements support for the pallet_balances module.

use core::marker::PhantomData;
use std::fmt::Debug;

use codec::{
    Decode,
    Encode,
};
use frame_support::{
    Parameter,
};
use sp_runtime::traits::{
    AtLeast32Bit,
    MaybeSerialize,
    Member,
};

use crate::frame::system::System;
use crate::sp_runtime::RuntimeAppPublic;
use crate::sp_runtime::traits::MaybeSerializeDeserialize;

/// Subset of Thea Config trait
#[module]
pub trait Thea: System {
    /// Authority identifier type
    type TheaId: Member
    + Parameter
    + RuntimeAppPublic
    + Default
    + MaybeSerializeDeserialize
    + Ord;
    /// The balance of an account.
    type Balance: Parameter
    + Member
    + AtLeast32Bit
    + codec::Codec
    + Default
    + Copy
    + MaybeSerialize
    + Debug
    + From<<Self as System>::BlockNumber>;
}

/// New Deposit Address event.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewDepositAddressRegisered<T: Thea> {
    /// Deposit address that was registered
    pub address: <T as System>::AccountId,
}

/// New Thea Public Key set
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ECDSAKeySet<T: Thea> {
    /// Deposit address that was registered
    pub set_id: u64,
    /// ECDSA Public Key
    pub public_key: sp_core::ecdsa::Public,
    /// To make rust happy
    _runtime: PhantomData<T>,
}

/// Thea Supported Networks
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Network {
    /// Bitcoin Mainnet
    BITCOIN,
    /// Not Supported
    NONE,
}

impl Default for Network {
    fn default() -> Self {
        Self::NONE
    }
}

/// Unsigned Thea payload
#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub struct UnsignedTheaPayload {
    /// Network Type
    pub network: Network,
    /// Payload for signing
    pub payload: [u8; 32],
}

/// Contains both payload and valid signature
#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub struct SignedTheaPayload {
    /// Unsigned Payload
    pub payload: UnsignedTheaPayload,
    /// Valid Signature
    pub signature: sp_core::ecdsa::Signature,
}

/// Get's signed paylaods
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct SignedPayloads<'a, T: Thea> {
    #[store(returns = Vec < SignedTheaPayload >)]
    /// Blocknumber to retrieve the `Vec<SignedTheaPayload>` for.
    pub block_number: &'a T::BlockNumber,
}

/// ValidatorSetId storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ValidatorSetId<T: Thea> {
    #[store(returns = u64)]
    /// just to make rustc happy
    _runtime: PhantomData<T>,
}

/// Authorities storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Authorities<T: Thea> {
    #[store(returns = Vec <T::TheaId>)]
    /// just to make rustc happy
    _runtime: PhantomData<T>,
}

/// Next Authorities storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct NextAuthorities<T: Thea> {
    #[store(returns = Vec <T::TheaId>)]
    /// just to make rustc happy
    _runtime: PhantomData<T>,
}

/// Get Thea Public Keys
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PublicKeys<'a, T: Thea> {
    #[store(returns = sp_core::ecdsa::Public)]
    /// Set id for which we need Thea public key
    pub set_id: &'a u64,
    /// to make rustc happy
    _runtime: PhantomData<T>,
}

/// Registered Deposit Address storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct RegisteredDepositAddress<'a, T: Thea> {
    #[store(returns = bool)]
    /// Set id for which we need Thea public key
    pub account: &'a T::AccountId,
}