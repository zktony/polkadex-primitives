use crate::ocex::{OCEXConfig, TradingPairConfig};
use crate::AssetId;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::H256;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IngressMessages<AccountId, Balance> {
    // Start Enclave
    StartEnclave(OCEXConfig<AccountId>),
    // Open Trading Pair
    OpenTradingPair(TradingPairConfig<Balance>),
    // Register User ( main, proxy)
    RegisterUser(AccountId, AccountId),
    // Main Acc, Assetid, Amount
    Deposit(AccountId, AssetId, Balance),
    // Main Acc, Proxy Account
    AddProxy(AccountId, AccountId),
    // Main Acc, Proxy Account
    RemoveProxy(AccountId, AccountId),
    // Enclave registration confirmation
    EnclaveRegistered(AccountId),
    // Shutdown Exchange
    Shutdown,
    // Close Trading Pair
    CloseTradingPair(TradingPairConfig<Balance>),
    // Latest snapshot (MerkelRoot, snapshot_no)
    LastestSnapshot(H256, u32)
}
