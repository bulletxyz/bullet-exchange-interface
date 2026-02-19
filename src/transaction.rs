//! Transaction types.

use crate::{define_enum, define_simple_type, define_struct};

/// Fix the Address type for this module.
pub type ExchangeCall = crate::message::CallMessage<crate::address::Address>;

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    sov_universal_wallet::UniversalWallet,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[borsh(use_discriminant = true)]
/// The top-level structure to be send to the Rollup.
pub enum Transaction {
    V0(Version0) = 0,
}

/// A transaction with a single signer.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    sov_universal_wallet::UniversalWallet,
)]
pub struct Version0 {
    /// The signature of the transaction.
    #[serde(with = "hex::serde")]
    #[sov_wallet(display = "hex")]
    pub signature: [u8; 64],
    #[serde(with = "hex::serde")]
    #[sov_wallet(display = "hex")]
    pub pub_key: [u8; 32],
    pub runtime_call: RuntimeCall,
    pub uniqueness: UniquenessData,
    pub details: TxDetails,
}

define_struct!(
    /// The transaction to be signed.
    struct UnsignedTransaction {
        runtime_call: RuntimeCall,
        uniqueness: UniquenessData,
        details: TxDetails,
    }
);

define_enum!(
    /// The enum to distinguish the rollup modules.
    enum RuntimeCall {
        Exchange(ExchangeCall) = 7,
    }
);

define_enum!(
    /// A nonce to detect replays.
    enum UniquenessData {
        Nonce(u64) = 0,
        Generation(u64) = 1,
    }
);

define_struct!(
    /// Metadata to be given to any transactions.
    struct TxDetails {
        max_priority_fee_bips: PriorityFeeBips,
        max_fee: Amount,
        gas_limit: Option<Gas>,
        chain_id: u64,
    }
);

define_simple_type!(Gas([u64; 2]) + Debug + sov_universal_wallet::UniversalWallet);
define_simple_type!(PriorityFeeBips(u64));
define_simple_type!(Amount(u128));
