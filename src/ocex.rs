use frame_support::BoundedVec;
use frame_support::traits::Get;
use codec::{Encode,Decode,MaxEncodedLen};
use scale_info::TypeInfo;
use crate::assets::AssetId;

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

#[derive(Encode,Decode, MaxEncodedLen,TypeInfo)]
pub struct TradingPairInfo<Balance>{
    base_asset: AssetId,
    quote_asset: AssetId,
    // Minimum size of trade
    minimum_trade_amount: Balance,
    minimum_withdrawal_amount: Balance,
    minimum_deposit_amount: Balance,
    maximum_withdrawal_amount: Balance,
    maximum_deposit_amount: Balance,
    base_withdrawal_fee: Balance,
    quote_withdrawal_fee: Balance,
}

impl<Balance> TradingPairInfo<Balance> {
    pub fn new(base_asset: AssetId,
               quote_asset: AssetId,
               minimum_trade_amount: Balance,
               minimum_withdrawal_amount: Balance,
               minimum_deposit_amount: Balance,
               maximum_withdrawal_amount: Balance,
               maximum_deposit_amount: Balance,
               base_withdrawal_fee: Balance,
               quote_withdrawal_fee: Balance,
    ) -> TradingPairInfo<Balance>{
        TradingPairInfo{
            base_asset,
            quote_asset,
            minimum_trade_amount,
            minimum_withdrawal_amount,
            minimum_deposit_amount,
            maximum_withdrawal_amount,
            maximum_deposit_amount,
            base_withdrawal_fee,
            quote_withdrawal_fee,
        }
    }
}
