use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::Zero;

#[derive(Copy, Clone, Encode, Decode, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FeeConfig<Balance: Zero> {
    pub(crate) maker_fraction: Balance,
    pub(crate) taker_fraction: Balance,
}

impl<Balance: Zero> Default for FeeConfig<Balance> {
    fn default() -> Self {
        Self {
            maker_fraction: Balance::zero(),
            taker_fraction: Balance::zero(),
        }
    }
}
