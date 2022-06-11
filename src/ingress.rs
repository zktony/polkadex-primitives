use crate::ocex::{OCEXConfig, TradingPairConfig};
use crate::AssetId;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IngressMessages<AccountId, Balance> {
    // Start Enclave
    StartEnclave(OCEXConfig<AccountId>),
    // Register Trading Pair
    RegisterTradingPair(TradingPairConfig<Balance>),
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
    // Enclave Shutdown request
    ShutdownEnclave(AccountId),
}
