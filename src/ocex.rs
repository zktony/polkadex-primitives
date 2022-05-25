use std::ops::Index;
use frame_support::BoundedVec;
use frame_support::traits::Get;
use codec::{Encode,Decode,MaxEncodedLen};
use scale_info::TypeInfo;
use crate::AssetId;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};


#[derive(Clone,Encode,Decode, MaxEncodedLen,TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IngressMessages<AccountId,Balance> {
    // Start Enclave
    StartEnclave(TradingPairInfo<AccountId,Balance>),
    // Register User ( main, proxy)
    RegisterUser(AccountId,AccountId),
    // Main Acc, Assetid, Amount
    Deposit(AccountId,AssetId,Balance),
    // Main Acc, Proxy Account
    AddProxy(AccountId,AccountId),
}

#[derive(Clone,Encode,Decode, MaxEncodedLen,TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EgressMessages<AccountId,Balance> {
    Withdrawal(Withdrawal<AccountId,Balance>),
    BalanceSnapShot(BalanceSnapshot),
    LMPData(LMPDataPoints)
}

#[derive(Clone,Encode,Decode, MaxEncodedLen,TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct LMPDataPoints {

}

#[derive(Clone,Encode,Decode, MaxEncodedLen,TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BalanceSnapshot {

}

#[derive(Clone,Encode,Decode, MaxEncodedLen,TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Withdrawal<AccountId,Balance> {
    pub main_account: AccountId,
    pub amount: Balance,
    pub asset: AssetId
}

#[derive(Encode,Decode, MaxEncodedLen,TypeInfo)]
#[scale_info(skip_type_params(ProxyLimit))]
pub struct AccountInfo<Account,ProxyLimit: Get<u32>> {
    proxies: BoundedVec<Account,ProxyLimit>
}

impl<Account,ProxyLimit: Get<u32>> AccountInfo<Account,ProxyLimit> {
    pub fn new(proxy: Account) -> AccountInfo<Account,ProxyLimit>{
        let mut proxies = BoundedVec::default();
        if let Err(()) = proxies.try_push(proxy){
            // It's okay to not handle this error since ProxyLimit is should be greater than one.
        }
        AccountInfo{ proxies }
    }

    // Adds a new proxy account
    pub fn add_proxy(&mut self, proxy: &Account) -> Result<(),()> {
        self.proxies.try_push(proxy)
    }

    // Removes a proxy account
    pub fn remove_proxy(&mut self, proxy: &Account){
        self.proxies.retain(| item | {
            item != proxy
        });
    }
}

#[derive(Clone, Encode,Decode, MaxEncodedLen,TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TradingPairInfo<AccountId,Balance>{
    pub base_asset: AssetId,
    pub quote_asset: AssetId,
    // Minimum size of trade
    pub minimum_trade_amount: Balance,
    pub maximum_trade_amount: Balance,
    pub minimum_withdrawal_amount: Balance,
    pub minimum_deposit_amount: Balance,
    pub maximum_withdrawal_amount: Balance,
    pub maximum_deposit_amount: Balance,
    pub base_withdrawal_fee: Balance,
    pub quote_withdrawal_fee: Balance,
    pub enclave_id: AccountId
}

impl<AccountId, Balance> TradingPairInfo<AccountId,Balance> {
    pub fn new(base_asset: AssetId,
               quote_asset: AssetId,
               minimum_trade_amount: Balance,
               maximum_trade_amount: Balance,
               minimum_withdrawal_amount: Balance,
               minimum_deposit_amount: Balance,
               maximum_withdrawal_amount: Balance,
               maximum_deposit_amount: Balance,
               base_withdrawal_fee: Balance,
               quote_withdrawal_fee: Balance,
               enclave_id: AccountId
    ) -> TradingPairInfo<AccountId,Balance>{
        TradingPairInfo{
            base_asset,
            quote_asset,
            minimum_trade_amount,
            maximum_trade_amount,
            minimum_withdrawal_amount,
            minimum_deposit_amount,
            maximum_withdrawal_amount,
            maximum_deposit_amount,
            base_withdrawal_fee,
            quote_withdrawal_fee,
            enclave_id
        }
    }
}
