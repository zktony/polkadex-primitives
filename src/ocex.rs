use crate::assets::AssetId;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::Get;
use frame_support::BoundedVec;
use scale_info::TypeInfo;
use std::collections::BTreeMap;

use crate::fees::FeeConfig;
use sp_runtime::traits::Zero;

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(ProxyLimit))]
pub struct AccountInfo<Account, Balance: Zero + Clone, ProxyLimit: Get<u32>> {
    pub main_account: Account,
    pub proxies: BoundedVec<Account, ProxyLimit>,
    pub nonce: u32,
    pub balances: BTreeMap<AssetId, (Balance, Balance)>,
    /// Trading Fee config
    pub fee_config: FeeConfig<Balance>,
}
impl<Account: PartialEq, Balance: Zero + Clone, ProxyLimit: Get<u32>>
    AccountInfo<Account, Balance, ProxyLimit>
{
    pub fn maker_fee_fraction(&self) -> Balance {
        self.fee_config.maker_fraction.clone()
    }
    pub fn taker_fee_fraction(&self) -> Balance {
        self.fee_config.taker_fraction.clone()
    }
}

impl<Account: PartialEq, Balance: Zero + Clone, ProxyLimit: Get<u32>>
    AccountInfo<Account, Balance, ProxyLimit>
{
    pub fn new(main_account_id: Account) -> AccountInfo<Account, Balance, ProxyLimit> {
        let proxies = BoundedVec::default();
        AccountInfo {
            main_account: main_account_id,
            proxies,
            nonce: 0,
            balances: BTreeMap::new(),
            fee_config: Default::default(),
        }
    }

    // Adds a new proxy account
    pub fn add_proxy(&mut self, proxy: Account) -> Result<(), ()> {
        self.proxies.try_push(proxy)
    }

    // Removes a proxy account
    pub fn remove_proxy(&mut self, proxy: &Account) {
        self.proxies.retain(|item| item != proxy);
    }
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OCEXConfig<AccountId> {
    pub enclave_id: AccountId,
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TradingPairConfig<Balance> {
    pub base_asset: AssetId,
    pub quote_asset: AssetId,
    pub min_trade_amount: Balance,
    pub max_trade_amount: Balance,
    pub min_order_qty: Balance,
    pub max_order_qty: Balance,
}
