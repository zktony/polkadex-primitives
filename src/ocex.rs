use crate::assets::AssetId;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::Get;
use frame_support::BoundedVec;
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::traits::Zero;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IngressMessages<AccountId, Balance> {
    // Start Enclave
    StartEnclave(TradingPairInfo<AccountId, Balance>),
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

/// Provides size of the unpadded report
pub struct UnpaddedReportSize;

impl Get<u32> for UnpaddedReportSize {
    fn get() -> u32 {
        432
    }
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(
    ProxyLimit,
    SnapshotAccLimit,
    WithdrawalLimit,
    UnpaddedReportSizeLimit
))]
pub enum EgressMessages<
    AccountId,
    Balance: Zero,
    ProxyLimit: Get<u32>,
    SnapshotAccLimit: Get<u32>,
    WithdrawalLimit: Get<u32>,
    UnpaddedReportSizeLimit: Get<u32>,
> {
    Withdrawal(Withdrawal<AccountId, Balance>),
    EnclaveSnapshot(
        EnclaveSnapshot<AccountId, Balance, ProxyLimit, SnapshotAccLimit, WithdrawalLimit>,
    ),
    RegisterEnclave(BoundedVec<u8, UnpaddedReportSizeLimit>),
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(ProxyLimit, SnapshotAccLimit, WithdrawalLimit))]
pub struct EnclaveSnapshot<
    Account,
    Balance: Zero,
    ProxyLimit: Get<u32>,
    SnapshotAccLimit: Get<u32>,
    WithdrawalLimit: Get<u32>,
> {
    /// Serial number of snapshot.
    pub snapshot_number: u32,
    /// List of accounts directly saved on chain, number of accounts bounded by SnapshotAccLimit
    pub accounts: BoundedVec<AccountInfo<Account, Balance, ProxyLimit>, SnapshotAccLimit>,
    /// Hash of the balance snapshot dump made by enclave. ( dump contains all the accounts in enclave )
    pub snapshot_whole_hash: H256,
    /// Sum of all q_finals of all lmp traders
    pub total_lmp_score: Balance,
    /// Withdrawals
    pub withdrawals: BoundedVec<Withdrawal<Account, Balance>, WithdrawalLimit>,
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Withdrawal<AccountId, Balance> {
    pub main_account: AccountId,
    pub amount: Balance,
    pub asset: AssetId,
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(ProxyLimit))]
pub struct AccountInfo<Account, Balance: Zero, ProxyLimit: Get<u32>> {
    pub proxies: BoundedVec<Account, ProxyLimit>,
    pub nonce: u32,
    /// quote asset reserved balance
    pub quote_reserved: Balance,
    /// quote asset free balance
    pub quote_free: Balance,
    /// base asset reserved balance
    pub base_reserved: Balance,
    /// base asset free balance
    pub base_free: Balance,
    /// Total Fees paid by this trader
    pub fee_paid_base_asset: Balance,
    pub fee_paid_quote_asset: Balance,
    pub q_final: Balance,
}

impl<Account: PartialEq, Balance: Zero, ProxyLimit: Get<u32>>
    AccountInfo<Account, Balance, ProxyLimit>
{
    pub fn new(proxy: Account) -> AccountInfo<Account, Balance, ProxyLimit> {
        let mut proxies = BoundedVec::default();
        if let Err(()) = proxies.try_push(proxy) {
            // It's okay to not handle this error since ProxyLimit is should be greater than one.
        }
        AccountInfo {
            proxies,
            nonce: 0,
            quote_reserved: Balance::zero(),
            quote_free: Balance::zero(),
            base_reserved: Balance::zero(),
            base_free: Balance::zero(),
            fee_paid_base_asset: Balance::zero(),
            fee_paid_quote_asset: Balance::zero(),
            q_final: Balance::zero(),
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
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TradingPairInfo<AccountId, Balance> {
    pub base_asset: AssetId,
    pub quote_asset: AssetId,
    // Minimum size of trade
    pub minimum_trade_amount: Balance,
    pub maximum_trade_amount: Balance,
    pub minimum_qty_amount: Balance, //minimum qty of a trade
    pub minimum_withdrawal_amount: Balance,
    pub minimum_deposit_amount: Balance,
    pub maximum_withdrawal_amount: Balance,
    pub maximum_deposit_amount: Balance,
    pub base_withdrawal_fee: Balance,
    pub quote_withdrawal_fee: Balance,
    pub enclave_id: AccountId,
    pub min_depth: Balance,
    pub max_spread: Balance,
}

impl<AccountId, Balance> TradingPairInfo<AccountId, Balance> {
    pub fn new(
        base_asset: AssetId,
        quote_asset: AssetId,
        minimum_trade_amount: Balance,
        maximum_trade_amount: Balance,
        minimum_qty_amount: Balance,
        minimum_withdrawal_amount: Balance,
        minimum_deposit_amount: Balance,
        maximum_withdrawal_amount: Balance,
        maximum_deposit_amount: Balance,
        base_withdrawal_fee: Balance,
        quote_withdrawal_fee: Balance,
        enclave_id: AccountId,
        min_depth: Balance,
        max_spread: Balance,
    ) -> TradingPairInfo<AccountId, Balance> {
        TradingPairInfo {
            base_asset,
            quote_asset,
            minimum_trade_amount,
            maximum_trade_amount,
            minimum_qty_amount,
            minimum_withdrawal_amount,
            minimum_deposit_amount,
            maximum_withdrawal_amount,
            maximum_deposit_amount,
            base_withdrawal_fee,
            quote_withdrawal_fee,
            enclave_id,
            min_depth,
            max_spread,
        }
    }
}
