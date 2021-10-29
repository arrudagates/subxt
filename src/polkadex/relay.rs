// This file is part of Polkadex.

// Copyright (C) 2020-2021 Polkadex oü.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Implements support for the pallet_balances module.

use bitcoin::types::{RawBlockHeader,H256Le};
use core::marker::PhantomData;
use std::fmt::Debug;

use codec::{
    Encode,
};


use crate::{
    frame::system::System,
};

/// Subset of Relay Config trait
#[module]
pub trait Relay: System {}

/// Initialize block header call
#[derive(Clone, Debug, Call, Encode)]
pub struct Initialize<T: Relay> {
    /// Raw Header
    raw_block_header: RawBlockHeader,
    /// Block height
    block_height: u32,
    /// Runtime marker
    _runtime: PhantomData<T>,
}

/// Initialize block header call
#[derive(Clone, Debug, Call, Encode)]
pub struct StoreBlockHeader<T: Relay> {
    /// Raw Header
    raw_block_header: RawBlockHeader,
    /// Runtime marker
    _runtime: PhantomData<T>,
}

/// Get Thea Public Keys
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct BestBlock<T: Relay> {
    #[store(returns = H256Le)]
    /// to make rustc happy
    _runtime: PhantomData<T>,
}

/// Get Thea Public Keys
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct BestBlockHeight<T: Relay> {
    #[store(returns = u32)]
    /// to make rustc happy
    _runtime: PhantomData<T>,
}