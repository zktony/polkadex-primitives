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
//! Low-level types used throughout the Substrate code.
pub extern crate alloc;
use crate::{AccountId, AssetId, Balance, SignedOrder};
use alloc::vec::Vec;
use codec::{Decode, Encode};
use frame_support::{sp_runtime::traits::AccountIdConversion, PalletId};

// Genesis Account constant should be kept up to date with OCEXGenesisAccount at https://github.com/Polkadex-Substrate/Polkadex/blob/main/runtime/src/lib.rs#L1536
const GENESIS_ACCOUNT: PalletId = PalletId(*b"polka/ga");

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub struct LinkedAccount {
    pub prev: AccountId,
    pub current: AccountId,
    pub next: Option<AccountId>,
    pub proxies: Vec<AccountId>,
}

impl LinkedAccount {
    pub fn from(prev: AccountId, current: AccountId) -> Self {
        LinkedAccount {
            prev,
            next: None,
            current,
            proxies: vec![],
        }
    }
}

impl Default for LinkedAccount {
    fn default() -> Self {
        LinkedAccount {
            prev: GENESIS_ACCOUNT.into_account(),
            current: GENESIS_ACCOUNT.into_account(),
            next: None,
            proxies: vec![],
        }
    }
}

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub struct PolkadexAccount {
    pub account: LinkedAccount,
    pub proof: Vec<Vec<u8>>,
}

#[derive(Debug, Encode, Decode, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct BalancesData {
    pub asset_id: AssetId,
    pub account_id: AccountId,
    pub free: Balance,
    pub reserved: Balance,
}

#[derive(Debug, Encode, Decode, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct NonceData {
    pub account_id: AccountId,
    pub nonce: u32,
}

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub struct OrderbookData {
    pub signed_order: SignedOrder,
}

#[derive(Debug, Encode, Decode)]
pub struct StorageData {
    pub balances: Vec<BalancesData>,
    pub nonce: Vec<NonceData>,
    pub orderbook: Vec<OrderbookData>,
}
