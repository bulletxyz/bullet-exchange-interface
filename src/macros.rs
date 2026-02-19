#[macro_export]
macro_rules! define_struct {
    ($(#[doc=$doc:literal])* struct $type_name:ident$(<$a:ident:$b:ident>)?{$($name:ident : $type:ty),* $(,)?} $(- $constraint:literal)?) => {
	$(#[doc=$doc])*
	#[derive(
	    Clone,
	    Debug,
	    Eq,
	    Hash,
	    Ord,
	    PartialEq,
	    PartialOrd,
	    borsh::BorshDeserialize,
            borsh::BorshSerialize,
            schemars::JsonSchema,
            serde::Deserialize,
            serde::Serialize,
            sov_universal_wallet::UniversalWallet,
	)]
	$(#[serde(bound = $constraint)]
          #[schemars(bound = $constraint)])*
	pub struct $type_name$(<$a:$b>)* {
	    $(pub $name : $type),*
	}
    };
    ($(#[doc=$doc:literal])* struct $type_name:ident$(<$a:ident:$b:ident>)?{$($name:ident : $type:ty),* $(,)?} $(- $constraint:literal)? $(+ $serde_opt:tt)*) => {
	$(#[doc=$doc])*
	#[derive(
	    Clone,
	    Debug,
	    Eq,
	    Hash,
	    Ord,
	    PartialEq,
	    PartialOrd,
	    borsh::BorshDeserialize,
            borsh::BorshSerialize,
            schemars::JsonSchema,
            serde::Deserialize,
            serde::Serialize,
            sov_universal_wallet::UniversalWallet,
	)]
	$(#[serde(bound = $constraint)]
          #[schemars(bound = $constraint)])*
	pub struct $type_name$(<$a:$b>)* {
	    $(pub $name : $type),*
	}
    }
}

#[macro_export]
macro_rules! define_simple_type {
    ($(#[doc=$doc:literal])* $name:ident($inner:ty)) => {
        define_simple_type!($(#[doc=$doc])* $name($inner) + Debug +  sov_universal_wallet::UniversalWallet);
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl Copy for $name {}
	impl std::str::FromStr for $name {
            type Err = $crate::error::ConfigError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let id = s
                    .parse()
                    .map_err(|_| $crate::error::ConfigError::FailedToParseInput {
                        input: s.to_string(),
                        reason: format!("Provided string value for {} cannot be converted to the underlying type ({})", stringify!($type_name), stringify!($inner)),
                    })?;

                Ok($name(id))
            }
        }
    };
    ($(#[doc=$doc:literal])* $name:ident($inner:ty) $(+ $derive:path)+) => {
	$(#[doc=$doc])*
        #[derive(
            Clone,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            borsh::BorshDeserialize,
            borsh::BorshSerialize,
            schemars::JsonSchema,
            serde::Deserialize,
            serde::Serialize,
	    $($derive),*
        )]
        #[serde(transparent)]
        pub struct $name(pub $inner);
    };
}

#[macro_export]
macro_rules! define_simple_enum {
    ($type_name:ident{$($(#[doc=$edoc:literal])* $name:ident $(= $value:literal)?),*$(,)?} $(+ $derive:path)*) => {
	$crate::define_enum!(enum $type_name{$($(#[doc=$edoc])* $name $(= $value)*),*} + Copy + strum::Display $(+ $derive)*);
    }
}

#[macro_export]
macro_rules! define_enum {
    ($(#[doc=$doc:literal])* enum $type_name:ident $(<$a:ident:$b:ident>)?{$($(#[doc=$edoc:literal])*$name:ident $({$($arg_name:ident: $arg_type:ty),+$(,)?})? $(= $value:literal)?),*$(,)?} $(- $constraint:literal)? $(+ $derive:path)*) => {
	$(#[doc=$doc])*
	#[derive(
	    Clone,
	    Debug,
	    Eq,
	    Hash,
	    Ord,
	    PartialEq,
	    PartialOrd,
	    borsh::BorshDeserialize,
	    borsh::BorshSerialize,
	    serde::Deserialize,
	    serde::Serialize,
            schemars::JsonSchema,
	    sov_universal_wallet::UniversalWallet,
	    $($derive),*
	)]
	#[repr(u8)]
	#[serde(rename_all = "snake_case")]
	#[borsh(use_discriminant = true)]
	$(#[serde(bound = $constraint)]
          #[schemars(bound = $constraint)])*
        pub enum $type_name$(<$a:$b>)* {
	    $($(#[doc=$edoc])*
	      $name $({$($arg_name: $arg_type),* })* $(= $value)*),*
	}
    };
    ($(#[doc=$doc:literal])* enum $type_name:ident $(<$a:ident:$b:ident>)?{$($(#[doc=$edoc:literal])*$name:ident ($arg_type:ty) $(= $value:literal)?),+$(,)?} $(- $constraint:literal)? $(+ $derive:path)*) => {
	$(#[doc=$doc])*
	#[derive(
	    Clone,
	    Debug,
	    Eq,
	    Hash,
	    Ord,
	    PartialEq,
	    PartialOrd,
	    borsh::BorshDeserialize,
	    borsh::BorshSerialize,
	    serde::Deserialize,
	    serde::Serialize,
            schemars::JsonSchema,
	    sov_universal_wallet::UniversalWallet,
	    $($derive),*
	)]
	#[repr(u8)]
	#[serde(rename_all = "snake_case")]
	#[borsh(use_discriminant = true)]
	$(#[serde(bound = $constraint)]
          #[schemars(bound = $constraint)])*
	pub enum $type_name$(<$a:$b>)* {
	    $($(#[doc=$edoc])*
	      $name ($arg_type) $(= $value)*),*
	}
    }
}
