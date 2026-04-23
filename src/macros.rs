/// Define a structure with a set of default derives.
#[macro_export]
macro_rules! define_struct {
    ($(#[$structmeta:meta])* struct $type_name:ident$(<$a:ident>)?{$($(#[$meta:meta])*$name:ident : $type:ty),* $(,)?}) => {
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
        )]
        #[cfg_attr(feature="schema", derive(sov_universal_wallet::UniversalWallet))]
        $(#[$structmeta])*
        pub struct $type_name$(<$a>)* {
            $($(#[$meta])*
              pub $name : $type),*
        }
    };
}

#[macro_export]
macro_rules! define_simple_type {
    ($(#[$enummeta:meta])* $name:ident($inner:ty)) => {
        define_simple_type!(
            $(#[$enummeta])*
		#[cfg_attr(feature="schema", derive(sov_universal_wallet::UniversalWallet))]
                $name($inner) + Default + Debug);
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
    ($(#[$enummeta:meta])* $name:ident($(#[$innermeta:meta])* $inner:ty) $(+ $derive:path)*) => {
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
        $(#[$enummeta])*
        pub struct $name(
	    $(#[$innermeta])*
	    pub $inner);
    };
}

#[macro_export]
macro_rules! define_simple_enum {
    ($(#[$enummeta:meta])* $type_name:ident{$($(#[$meta:meta])* $name:ident $(= $value:literal)?),*$(,)?}) => {
        $crate::define_enum!(
            $(#[$enummeta])*
            #[derive(Copy, strum::Display, strum::FromRepr)]
            enum $type_name{$($(#[$meta])* $name $(= $value)*),*});
    }
}

#[macro_export]
macro_rules! define_enum {
    ($(#[$enummeta:meta])* enum $type_name:ident $(<$a:ident>)?{$($(#[$meta:meta])*$name:ident $({$($arg_name:ident: $arg_type1:ty),*$(,)?})? $(($($arg_type2:ty),*))? $(= $value:literal)?),+$(,)?}) => {
        $(#[$enummeta])*
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
            strum::AsRefStr,
        )]
	#[cfg_attr(feature="schema", derive(sov_universal_wallet::UniversalWallet))]
	#[derive(strum::EnumDiscriminants)]
	#[strum_discriminants(derive(strum::EnumIter, strum::EnumString, strum::Display))]
        #[repr(u8)]
        #[serde(rename_all = "snake_case")]
        #[borsh(use_discriminant = true)]
        pub enum $type_name$(<$a>)* {
            $($(#[$meta])*
              $name $({$($arg_name: $arg_type1),* })* $(($($arg_type2),*))* $(= $value)*),*
        }
    }
}
