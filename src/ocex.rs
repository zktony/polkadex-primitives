use std::mem;
use frame_support::BoundedVec;
use frame_support::traits::Get;
use codec::{Encode,Decode,MaxEncodedLen};
use scale_info::TypeInfo;
use crate::assets::AssetId;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};


#[derive(Clone,Encode,Decode, MaxEncodedLen,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IngressMessages<AccountId> {
    // Start Enclave
    StartEnclave(TradingPairInfo<AccountId>),
    // Register User ( main, proxy)
    RegisterUser(AccountId,AccountId),
    // Main Acc, Assetid, Amount
    Deposit(AccountId,AssetId,[u8;mem::size_of::<u128>()]),
    // Main Acc, Proxy Account
    AddProxy(AccountId,AccountId),
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
}

#[derive(Clone, Encode,Decode, MaxEncodedLen,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TradingPairInfo<AccountId>{
    pub base_asset: AssetId,
    pub quote_asset: AssetId,
    // Minimum size of trade
    pub minimum_trade_amount: [u8;mem::size_of::<u128>()],
    pub maximum_trade_amount: [u8;mem::size_of::<u128>()],
    pub minimum_withdrawal_amount: [u8;mem::size_of::<u128>()],
    pub minimum_deposit_amount: [u8;mem::size_of::<u128>()],
    pub maximum_withdrawal_amount: [u8;mem::size_of::<u128>()],
    pub maximum_deposit_amount: [u8;mem::size_of::<u128>()],
    pub base_withdrawal_fee: [u8;mem::size_of::<u128>()],
    pub quote_withdrawal_fee: [u8;mem::size_of::<u128>()],
    pub enclave_id: AccountId,
}

impl<AccountId> TradingPairInfo<AccountId> {
    pub fn new(base_asset: AssetId,
               quote_asset: AssetId,
               minimum_trade_amount: u128,
               maximum_trade_amount: u128,
               minimum_withdrawal_amount: u128,
               minimum_deposit_amount: u128,
               maximum_withdrawal_amount: u128,
               maximum_deposit_amount: u128,
               base_withdrawal_fee: u128,
               quote_withdrawal_fee: u128,
               enclave_id: AccountId
    ) -> TradingPairInfo<AccountId>{
        TradingPairInfo{
            base_asset,
            quote_asset,
            minimum_trade_amount: minimum_trade_amount.to_be_bytes(),
            maximum_trade_amount: maximum_trade_amount.to_be_bytes(),
            minimum_withdrawal_amount: minimum_withdrawal_amount.to_be_bytes(),
            minimum_deposit_amount: minimum_deposit_amount.to_be_bytes(),
            maximum_withdrawal_amount:maximum_withdrawal_amount.to_be_bytes(),
            maximum_deposit_amount: maximum_deposit_amount.to_be_bytes(),
            base_withdrawal_fee:base_withdrawal_fee.to_be_bytes(),
            quote_withdrawal_fee: quote_withdrawal_fee.to_be_bytes(),
            enclave_id
        }
    }
}
