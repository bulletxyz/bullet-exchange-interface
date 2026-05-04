//! Base58 address.
#[derive(
    Clone, Eq, Hash, Ord, PartialEq, PartialOrd, borsh::BorshDeserialize, borsh::BorshSerialize,
)]
#[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
pub struct Address(#[cfg_attr(feature = "schema", sov_wallet(display = "base58"))] pub [u8; 32]);

impl Copy for Address {}

impl serde::Serialize for Address {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serde::Serialize::serialize(&self.0, serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String as serde::Deserialize<'_>>::deserialize(deserializer)?;
            s.parse().map_err(serde::de::Error::custom)
        } else {
            let bytes = <[u8; 32] as serde::Deserialize<'_>>::deserialize(deserializer)?;
            Ok(Self(bytes))
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::default();
        // keep at least 32-chars
        bullet_bs58::encode32_append(&self.0, 32, &mut res);
        write!(f, "{res}")
    }
}

impl schemars::JsonSchema for Address {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Address".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "pattern": "[1-9A-HJ-NP-Za-km-z]{32,44}",
            "description": "Address",
        })
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let key: [u8; 32] = value.try_into().map_err(|_| {
            format!(
                "Invalid base58 address. Got {} instead of 32 bytes.",
                value.len()
            )
        })?;
        Ok(Self(key))
    }
}

impl std::str::FromStr for Address {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(x) = bullet_bs58::parse32(s.as_bytes()) else {
            return Err(format!("Invalid base58 address `{s}`."));
        };
        Ok(Self(x))
    }
}
