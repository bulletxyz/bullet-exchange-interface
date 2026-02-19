//! Argument types for vault operations.

use crate::decimals::PositiveDecimal;
use crate::define_struct;

define_struct! {
    struct UpdateVaultConfigArgs {
        deposit_limit: Option<PositiveDecimal>,
        withdraw_lockup_period_hours: Option<u8>,
        profit_share_percentage: Option<u8>,
    }
}
