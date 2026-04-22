#[cfg(feature = "schema")]
use sov_universal_wallet::schema::{IndexLinking, Item, Link, Primitive, Schema, UniversalWallet};

use crate::define_simple_type;

define_simple_type!(CustomString(String) + Debug);

#[cfg(feature = "schema")]
impl UniversalWallet for CustomString {
    fn scaffold() -> Item<IndexLinking> {
        Item::Atom(Primitive::String)
    }
    fn get_child_links(_schema: &mut Schema) -> Vec<Link> {
        Vec::new()
    }
}

impl CustomString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for CustomString {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl From<&str> for CustomString {
    fn from(v: &str) -> Self {
        Self(v.to_string())
    }
}

impl std::fmt::Display for CustomString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
