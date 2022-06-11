use crate::snapshot::{EnclaveAccountInfoDump, EnclaveSnapshot};
use crate::Signature;
use frame_support::traits::Get;
use frame_support::BoundedVec;
use sp_runtime::traits::Zero;

use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(SnapshotAccLimit, WithdrawalLimit))]
pub enum EgressMessages<
    AccountId: Ord,
    Balance: Zero + Clone,
    ProxyLimit: Get<u32>,
    WithdrawalLimit: Get<u32>,
> {
    EnclaveAccountDump(
        EnclaveAccountInfoDump<AccountId, Balance, ProxyLimit>,
        Signature,
    ),
    EnclaveSnapshot(
        EnclaveSnapshot<AccountId, Balance, WithdrawalLimit>,
        Signature,
    ),
    RegisterEnclave(BoundedVec<u8, UnpaddedReportSize>),
}

/// Provides size of the unpadded report
pub struct UnpaddedReportSize;
impl Get<u32> for UnpaddedReportSize {
    fn get() -> u32 {
        432
    }
}
