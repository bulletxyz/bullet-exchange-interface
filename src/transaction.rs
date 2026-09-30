//! Transaction types.

use crate::{define_enum, define_simple_type, define_struct};

pub mod bank;
pub mod warp;

/// The maxium size of each transaction.
pub const MAX_TX_SIZE: usize = 8000;

/// Fix the Address type for this module.
pub type ExchangeCall = crate::message::CallMessage<crate::address::Address>;
pub type BankCall = bank::CallMessage<crate::address::Address>;
pub type WarpCall = warp::CallMessage<crate::address::Address>;

define_simple_type!(
    /// A 32-byte Warp route or recipient, encoded as 0x-prefixed hex in JSON.
    #[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
    WarpBytes32(
        #[serde(with = "crate::transaction::serde_hex_32")]
        #[schemars(with = "String")]
        #[cfg_attr(feature = "schema", sov_wallet(display = "hex"))]
        [u8; 32]
    ) + Copy + Debug
);

define_simple_type!(
    /// A 20-byte Ethereum address (Hyperlane validator), encoded as 0x-prefixed hex in JSON.
    /// Mirrors the runtime's `EthAddress = HexString<[u8; 20]>`.
    #[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
    WarpBytes20(
        #[serde(with = "crate::transaction::serde_hex_20")]
        #[schemars(with = "String")]
        #[cfg_attr(feature = "schema", sov_wallet(display = "hex"))]
        [u8; 20]
    ) + Copy + Debug
);

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
    /// The account to execute as. `None` executes as the signer's default address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_override: Option<crate::address::Address>,
}

define_struct! {
    /// The transaction to be signed.
    struct UnsignedTransaction {
        runtime_call: RuntimeCall,
        uniqueness: UniquenessData,
        details: TxDetails,
        /// The account to execute as. `None` executes as the signer's default address.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        address_override: Option<crate::address::Address>,
    }
}

impl UnsignedTransaction {
    /// The payload a single signer signs for `chain_hash`.
    pub fn signing_payload(&self, chain_hash: &[u8; 32]) -> TransactionSigningPayload {
        TransactionSigningPayload::V0(TransactionSigningPayloadV0 {
            runtime_call: self.runtime_call.clone(),
            uniqueness: self.uniqueness.clone(),
            details: self.details.clone(),
            address_override: self.address_override,
            chain_hash: *chain_hash,
        })
    }

    /// The bytes a single signer signs for `chain_hash`: the Borsh encoding of
    /// [`TransactionSigningPayload::V0`]. `details.chain_hash_fragment` must be
    /// [`chain_hash_fragment`] of the same hash or the rollup rejects the transaction.
    pub fn signing_bytes(&self, chain_hash: &[u8; 32]) -> Vec<u8> {
        borsh::to_vec(&self.signing_payload(chain_hash))
            .expect("serializing a signing payload to a Vec cannot fail")
    }

    /// Wraps this transaction and a signature over [`Self::signing_bytes`] into the wire format.
    pub fn into_signed(self, pub_key: [u8; 32], signature: [u8; 64]) -> Transaction {
        Transaction::V0(Version0 {
            signature,
            pub_key,
            runtime_call: self.runtime_call,
            uniqueness: self.uniqueness,
            details: self.details,
            address_override: self.address_override,
        })
    }
}

/// The data a signer commits to. The Borsh encoding of this enum is what gets signed, so the
/// version discriminant (`0` for V0) is the first signed byte and the chain hash is the last 32.
#[derive(Clone, Debug, Eq, PartialEq, borsh::BorshDeserialize, borsh::BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
#[repr(u8)]
pub enum TransactionSigningPayload {
    V0(TransactionSigningPayloadV0) = 0,
}

impl TransactionSigningPayload {
    /// Parses the bytes produced by [`UnsignedTransaction::signing_bytes`].
    pub fn from_signing_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        borsh::from_slice(bytes)
    }
}

/// The single-signer signing payload: the unsigned transaction plus the chain hash.
#[derive(Clone, Debug, Eq, PartialEq, borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct TransactionSigningPayloadV0 {
    pub runtime_call: RuntimeCall,
    pub uniqueness: UniquenessData,
    pub details: TxDetails,
    pub address_override: Option<crate::address::Address>,
    pub chain_hash: [u8; 32],
}

impl TransactionSigningPayloadV0 {
    /// Recovers the unsigned transaction this payload was built from.
    pub fn into_unsigned_transaction(self) -> UnsignedTransaction {
        UnsignedTransaction {
            runtime_call: self.runtime_call,
            uniqueness: self.uniqueness,
            details: self.details,
            address_override: self.address_override,
        }
    }
}

/// The 64-bit fragment of a chain hash that transaction details commit to: the first eight
/// bytes, little-endian, so Borsh writes them back unchanged.
pub const fn chain_hash_fragment(chain_hash: &[u8; 32]) -> u64 {
    u64::from_le_bytes([
        chain_hash[0],
        chain_hash[1],
        chain_hash[2],
        chain_hash[3],
        chain_hash[4],
        chain_hash[5],
        chain_hash[6],
        chain_hash[7],
    ])
}

define_enum! {
    /// The enum to distinguish the rollup modules.
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
    enum RuntimeCall {
        Bank(BankCall) = 2,
        Exchange(ExchangeCall) = 7,
        Warp(WarpCall) = 15,
    }
}

define_enum! {
    /// A nonce to detect replays.
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
    enum UniquenessData {
        Nonce(u64) = 0,
        Generation(u64) = 1,
        Window(u64) = 2,
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
        /// [`chain_hash_fragment`] of the chain hash the signer committed to. The rollup
        /// serializes it as a decimal string in JSON.
        #[serde(with = "serde_u64_decimal_string")]
        #[schemars(with = "String")]
        #[cfg_attr(feature = "schema", sov_wallet(hidden))]
        chain_hash_fragment: u64,
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

/// Serde for [`TxDetails::chain_hash_fragment`]: the rollup reads it as a decimal string in JSON.
mod serde_u64_decimal_string {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.collect_str(value)
        } else {
            serializer.serialize_u64(*value)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let value = String::deserialize(deserializer)?;
            value.parse::<u64>().map_err(serde::de::Error::custom)
        } else {
            u64::deserialize(deserializer)
        }
    }
}

mod serde_amount_decimal_string {
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::transaction::Amount;

    pub fn serialize<S>(amount: &Amount, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&amount.0.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Amount, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .parse::<u128>()
            .map(Amount)
            .map_err(serde::de::Error::custom)
    }
}

/// Serde for the Bank `TokenId` (`[u8; 32]`): the rollup encodes a token id as a
/// Bech32m `token_…` string (matching `sov_bank::TokenId` / `impl_hash32_type!`),
/// not a byte array. Emitting the raw bytes fails deserialization with
/// "invalid type: sequence, expected a string".
mod serde_token_id_bech32m {
    use bech32::primitives::decode::CheckedHrpstring;
    use bech32::{Bech32m, Hrp};
    use serde::{Deserialize, Deserializer, Serializer};

    const HRP: &str = "token_";

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hrp = Hrp::parse(HRP).map_err(serde::ser::Error::custom)?;
        let s = bech32::encode::<Bech32m>(hrp, bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let checked = CheckedHrpstring::new::<Bech32m>(&value).map_err(serde::de::Error::custom)?;
        if checked.hrp().as_str() != HRP {
            return Err(serde::de::Error::custom(format!(
                "token id hrp must be `{HRP}`, got `{}`",
                checked.hrp().as_str()
            )));
        }
        let bytes: Vec<u8> = checked.byte_iter().collect();
        bytes.try_into().map_err(|v: Vec<u8>| {
            serde::de::Error::custom(format!("token id must be 32 bytes, got {}", v.len()))
        })
    }
}

mod serde_hex_32 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value.strip_prefix("0x").unwrap_or(value.as_str());
        let mut bytes = [0; 32];
        hex::decode_to_slice(value, &mut bytes).map_err(serde::de::Error::custom)?;

        Ok(bytes)
    }
}

mod serde_hex_20 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 20], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 20], D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value.strip_prefix("0x").unwrap_or(value.as_str());
        let mut bytes = [0; 20];
        hex::decode_to_slice(value, &mut bytes).map_err(serde::de::Error::custom)?;

        Ok(bytes)
    }
}

/// Serde for `Option<Amount>` fields that must be decimal strings (or null) in
/// JSON — the `Update` warp call uses these for optional rate-limit changes.
mod serde_amount_decimal_string_opt {
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::transaction::Amount;

    pub fn serialize<S>(amount: &Option<Amount>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match amount {
            Some(a) => serializer.serialize_some(&a.0.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Amount>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            Some(s) => s
                .parse::<u128>()
                .map(Amount)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unsigned(chain_hash: &[u8; 32]) -> UnsignedTransaction {
        UnsignedTransaction {
            runtime_call: RuntimeCall::Bank(BankCall::Mint {
                coins: bank::Coins {
                    amount: Amount(5),
                    token_id: bank::config_gas_token_id(),
                },
                mint_to_address: crate::address::Address([3u8; 32]),
            }),
            uniqueness: UniquenessData::Window(7),
            details: TxDetails {
                max_priority_fee_bips: PriorityFeeBips(1),
                max_fee: Amount(2),
                gas_limit: None,
                chain_hash_fragment: chain_hash_fragment(chain_hash),
            },
            address_override: None,
        }
    }

    #[test]
    fn chain_hash_fragment_is_first_eight_bytes_little_endian() {
        let mut chain_hash = [0xaa; 32];
        chain_hash[..8].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(chain_hash_fragment(&chain_hash), 0x0807_0605_0403_0201);
        assert_eq!(
            borsh::to_vec(&chain_hash_fragment(&chain_hash)).unwrap(),
            chain_hash[..8]
        );
    }

    #[test]
    fn signing_bytes_are_versioned_unsigned_transaction_plus_chain_hash() {
        let chain_hash = [0x11; 32];
        let tx = unsigned(&chain_hash);
        let bytes = tx.signing_bytes(&chain_hash);
        let mut expected = vec![0u8];
        expected.extend(borsh::to_vec(&tx).unwrap());
        expected.extend_from_slice(&chain_hash);
        assert_eq!(bytes, expected);
        let TransactionSigningPayload::V0(payload) =
            TransactionSigningPayload::from_signing_bytes(&bytes).unwrap();
        assert_eq!(payload.chain_hash, chain_hash);
        assert_eq!(payload.into_unsigned_transaction(), tx);
    }

    #[test]
    fn wire_format_is_v0_header_plus_unsigned_transaction() {
        let chain_hash = [0x22; 32];
        let tx = unsigned(&chain_hash);
        let signed = tx.clone().into_signed([9u8; 32], [8u8; 64]);
        let mut expected = vec![0u8];
        expected.extend_from_slice(&[8u8; 64]);
        expected.extend_from_slice(&[9u8; 32]);
        expected.extend(borsh::to_vec(&tx).unwrap());
        assert_eq!(borsh::to_vec(&signed).unwrap(), expected);
    }

    #[test]
    fn json_encodes_fragment_as_decimal_string_and_omits_absent_override() {
        let chain_hash = [0x33; 32];
        let tx = unsigned(&chain_hash);
        let json = serde_json::to_value(&tx).unwrap();
        assert_eq!(
            json["details"]["chain_hash_fragment"],
            chain_hash_fragment(&chain_hash).to_string()
        );
        assert!(json.get("address_override").is_none());
        let roundtrip: UnsignedTransaction = serde_json::from_value(json).unwrap();
        assert_eq!(roundtrip, tx);
    }
}
