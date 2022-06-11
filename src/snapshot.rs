use crate::ocex::AccountInfo;
use crate::withdrawal::Withdrawal;
use frame_support::BoundedVec;
use sp_core::H256;
use sp_runtime::traits::Zero;
use sp_std::collections::btree_map::BTreeMap;

use codec::{Decode, Encode};
use frame_support::traits::Get;
use scale_info::TypeInfo;

/// Provides maximum number of accounts possible in enclave data dump
pub struct AccountInfoDumpLimit;
impl Get<u32> for AccountInfoDumpLimit {
    fn get() -> u32 {
        10000000
    }
}

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EnclaveAccountInfoDump<AccountId: Ord, Balance: Zero + Clone, ProxyLimit: Get<u32>> {
    /// Serial number of snapshot.
    pub snapshot_number: u32,
    /// All Accounts present in enclave
    pub accounts: BTreeMap<AccountId, AccountInfo<AccountId, Balance, ProxyLimit>>,
}

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(SnapshotAccLimit, WithdrawalLimit))]
pub struct EnclaveSnapshot<Account, Balance: Zero + Clone, WithdrawalLimit: Get<u32>> {
    /// Serial number of snapshot.
    pub snapshot_number: u32,
    /// Hash of the balance snapshot dump made by enclave. ( dump contains all the accounts in enclave )
    pub merkle_root: H256,
    /// Withdrawals
    pub withdrawals: BoundedVec<Withdrawal<Account, Balance>, WithdrawalLimit>,
    // TODO: Add base and quote fees collected by the exchange.
}

impl<Account, Balance: Zero + Clone, WithdrawalLimit: Get<u32>> PartialEq
    for EnclaveSnapshot<Account, Balance, WithdrawalLimit>
{
    fn eq(&self, other: &Self) -> bool {
        self.snapshot_number == other.snapshot_number
    }
}
