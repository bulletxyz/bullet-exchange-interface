//! Transaction types.

use crate::{define_enum, define_simple_type, define_struct};

/// The maxium size of each transaction.
pub const MAX_TX_SIZE: usize = 8000;

/// Fix the Address type for this module.
pub type ExchangeCall = crate::message::CallMessage<crate::address::Address>;
pub type BankCall = bank::CallMessage<crate::address::Address>;

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
)]
#[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[borsh(use_discriminant = true)]
/// The top-level structure to be send to the Rollup.
#[non_exhaustive]
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
)]
#[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
pub struct Version0 {
    /// The signature of the transaction.
    #[serde(with = "hex::serde")]
    #[cfg_attr(feature = "schema", sov_wallet(display = "hex"))]
    pub signature: [u8; 64],
    #[serde(with = "hex::serde")]
    #[cfg_attr(feature = "schema", sov_wallet(display = "hex"))]
    pub pub_key: [u8; 32],
    pub runtime_call: RuntimeCall,
    pub uniqueness: UniquenessData,
    pub details: TxDetails,
}

define_struct! {
    /// The transaction to be signed.
    struct UnsignedTransaction {
        runtime_call: RuntimeCall,
        uniqueness: UniquenessData,
        details: TxDetails,
    }
}

define_enum! {
    /// The enum to distinguish the rollup modules.
    #[non_exhaustive]
    enum RuntimeCall {
        Bank(BankCall) = 2,
        Exchange(ExchangeCall) = 7,
    }
}

define_enum! {
    /// A nonce to detect replays.
    #[non_exhaustive]
    enum UniquenessData {
        Nonce(u64) = 0,
        Generation(u64) = 1,
    }
}

define_struct! {
    /// Metadata to be given to any transactions.
    struct TxDetails {
        max_priority_fee_bips: PriorityFeeBips,
        /// The max fee one is willing to pay for this transaction.
        max_fee: Amount,
        /// Optionally limit the number of gas to be used.
        gas_limit: Option<Gas>,
        /// The chain-id from the schema.
        chain_id: u64,
    }
}

define_simple_type!(
    #[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
    Gas([u64; 2])
        + Debug
);
define_simple_type!(PriorityFeeBips(u64));
define_simple_type!(Amount(u128));
#[cfg(feature = "schema")]
impl sov_universal_wallet::ty::IntegerDisplayable for Amount {
    fn integer_type() -> sov_universal_wallet::ty::IntegerType {
        sov_universal_wallet::ty::IntegerType::u128
    }
}

pub mod bank {
    use crate::string::CustomString;
    use crate::transaction::Amount;
    use crate::{define_enum, define_simple_type, define_struct};

    define_simple_type!(
        #[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
        TokenId(
            #[cfg_attr(
                feature = "schema",
                sov_wallet(display(bech32m(prefix = "token_id_prefix()")))
            )]
            [u8; 32]
        ) + Debug
    );

    #[cfg(feature = "schema")]
    fn token_id_prefix() -> &'static str {
        "token_"
    }
    define_enum! {
        /// CallMessage for the Bank module.
        #[non_exhaustive]
        enum CallMessage<Address> {
            #[cfg_attr(feature="schema", sov_wallet(show_as = "Transfer to address {} {} with memo `{}`."))]
            TransferWithMemo {
                to: Address,
                coins: Coins,
                memo: CustomString,
            } = 6,
        }
    }
    define_struct! {
        #[cfg_attr(feature="schema", sov_wallet(show_as = "{} coins of token ID {}"))]
        struct Coins {
            #[cfg_attr(feature="schema", sov_wallet(fixed_point(from_field(1, offset = 31))))]
            amount: Amount,
            token_id: TokenId,
        }
    }
}
