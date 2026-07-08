use crate::string::CustomString;
use crate::transaction::Amount;
use crate::{define_enum, define_simple_type, define_struct};

define_simple_type!(
    #[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
    TokenId(
        #[cfg_attr(feature = "schema", sov_wallet(display(bech32m(prefix = "token_id_prefix()"))))]
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
    #[strum_discriminants(non_exhaustive)]
    enum CallMessage<Address> {
        Mint {
            coins: Coins,
            mint_to_address: Address,
        } = 3,
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
        #[cfg_attr(feature="schema",
           sov_wallet(fixed_point(from_field(1, offset = 31,
                             override_eq = config_gas_token_id().0,
                             override_decimals = config_gas_token_decimals()))))
        ]
        amount: Amount,
        token_id: TokenId,
    }
}

pub fn config_gas_token_id() -> TokenId {
    TokenId([
        153, 62, 252, 188, 142, 200, 250, 132, 195, 171, 34, 22, 170, 139, 109, 60, 134, 53, 48,
        11, 99, 82, 21, 138, 87, 53, 145, 193, 40, 198, 52, 89,
    ])
}

/// The number of decimal places of the rollup's gas token.
pub fn config_gas_token_decimals() -> u8 {
    9
}
