//! The fast-bridge deposit memo: where a depositor asks their bridged funds to land.
//!
//! A Solana depositor may attach a memo to a Hyperlane warp transfer, in the
//! `TokenMessage.metadata` tail. The fast-bridge operator decodes it to choose the
//! beneficiary and the destination bucket, instead of defaulting to the depositor's
//! own wallet.
//!
//! The format lives here, rather than in the operator, so the encoder (the SDK,
//! which depositors use) and the decoder (the operator, which pays out) are the
//! same definition. If the two could drift, the operator would decode the SDK's
//! bytes as [`MemoDecodeError::Invalid`] and silently credit the signer instead of
//! the intended recipient, which is the quietest possible way to pay the wrong
//! person.
//!
//! Plain borsh rather than `define_struct!`: a memo never flows through
//! `CallMessage`, so it needs no schema registration.
//!
//! # Wire format
//!
//! ```text
//! version(1) | to_address(32) | Option<u8> sub_account_index | BalanceBucket
//! ```
//!
//! 35 bytes with no sub-account and a unit bucket, 38 with both a sub-account and
//! `Iso` (whose `MarketId` is two further little-endian bytes).

use borsh::{BorshDeserialize, BorshSerialize};

use crate::address::Address;
use crate::message::TransferEndpoint;

/// Longest memo accepted. The warp `TokenMessage.metadata` tail is unbounded on
/// chain and fully depositor-controlled, so a decoder that did not bound it would
/// let anyone choose how much the operator allocates.
pub const MEMO_MAX_LEN: usize = 256;

/// A deposit memo, tagged by version.
///
/// The discriminant *is* the leading version byte: borsh writes it on the way out
/// and rejects an unrecognised one on the way back, so there is no separate version
/// handling to keep in step. Adding a variant here is the only way to extend the
/// format, and the compiler then points at every site that must decide what to do
/// with it — which is the point, since silently treating a newer memo as an older
/// one would pay the wrong person.
#[derive(Debug, Clone, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[repr(u8)]
#[borsh(use_discriminant = true)]
pub enum Memo {
    V1(MemoV1) = 1,
}

/// A v1 memo: who to pay, and into which of their balances.
///
/// [`TransferEndpoint`] is reused verbatim from the exchange call it ends up in,
/// so a memo cannot name a destination the exchange would refuse to honour.
#[derive(Debug, Clone, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub struct MemoV1 {
    pub to_address: Address,
    pub to: TransferEndpoint,
}

/// Why a memo did not yield a destination.
///
/// Every variant means the same thing to the operator — fall back to the
/// depositor — but they are distinguished so the difference is visible in metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoDecodeError {
    /// No memo at all. The overwhelmingly common case, and not a problem.
    Absent,
    /// Longer than [`MEMO_MAX_LEN`].
    TooLong,
    /// Present but unusable: unknown version, malformed body, trailing bytes, or
    /// an unspendable destination.
    Invalid,
}

/// Encode a memo for a depositor to attach to their transfer.
pub fn encode(memo: &Memo) -> Vec<u8> {
    borsh::to_vec(memo).expect("writing to a Vec cannot fail")
}

/// Decode a memo taken from a warp `TokenMessage.metadata` tail.
///
/// Rejects rather than guesses, because the alternative to a clean rejection is
/// paying someone the depositor did not name.
///
/// Well-formedness only: whether the named destination is *creditable right now* is
/// the exchange's answer, not this one's. An `Iso` market this chain has never heard
/// of is accepted here on purpose; `an_unknown_market_id_is_left_to_the_exchange`
/// below says why.
pub fn decode(memo: &[u8]) -> Result<Memo, MemoDecodeError> {
    if memo.is_empty() {
        return Err(MemoDecodeError::Absent);
    }
    if memo.len() > MEMO_MAX_LEN {
        return Err(MemoDecodeError::TooLong);
    }
    // `from_slice` rejects an unknown version byte, trailing bytes, and an unknown
    // `BalanceBucket` discriminant alike — the decoder never invents a field it did
    // not read.
    let decoded: Memo = borsh::from_slice(memo).map_err(|_| MemoDecodeError::Invalid)?;
    let Memo::V1(ref v1) = decoded;
    // The zero address is unspendable, so honouring it would burn the deposit.
    if v1.to_address.0 == [0u8; 32] {
        return Err(MemoDecodeError::Invalid);
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BalanceBucket, MarketId};

    fn memo(sub_account_index: Option<u8>, balance: BalanceBucket) -> Memo {
        Memo::V1(MemoV1 {
            to_address: Address([0xAB; 32]),
            to: TransferEndpoint {
                sub_account_index,
                balance,
            },
        })
    }

    /// The byte layout is the contract between the SDK encoder and the operator
    /// decoder. Pinned here, and mirrored by the SDK's own tests.
    #[test]
    fn wire_layout_is_version_then_address_then_endpoint() {
        let bytes = encode(&memo(None, BalanceBucket::Bank));
        assert_eq!(bytes.len(), 1 + 32 + 1 + 1);
        assert_eq!(bytes[0], 1, "the V1 discriminant is the version byte");
        assert_eq!(&bytes[1..33], &[0xAB; 32]);
        assert_eq!(bytes[33], 0, "Option::None tag");
        assert_eq!(bytes[34], 0, "BalanceBucket::Bank");

        let bytes = encode(&memo(Some(2), BalanceBucket::Iso(MarketId(7))));
        assert_eq!(bytes.len(), 1 + 32 + 2 + 3);
        assert_eq!(bytes[33], 1, "Option::Some tag");
        assert_eq!(bytes[34], 2, "sub_account_index");
        assert_eq!(bytes[35], 3, "BalanceBucket::Iso");
        assert_eq!(&bytes[36..38], &7u16.to_le_bytes(), "MarketId is u16 LE");
    }

    #[test]
    fn round_trips_every_bucket_shape() {
        for m in [
            memo(None, BalanceBucket::Bank),
            memo(None, BalanceBucket::Cross),
            memo(Some(0), BalanceBucket::Spot),
            memo(Some(31), BalanceBucket::Iso(MarketId(0))),
        ] {
            assert_eq!(decode(&encode(&m)), Ok(m));
        }
    }

    #[test]
    fn empty_is_absent_but_a_bare_version_byte_is_invalid() {
        assert_eq!(decode(&[]), Err(MemoDecodeError::Absent));
        // Present and addressed to us, just unusable — the depositor tried to say
        // something, so this is not the same as saying nothing.
        assert_eq!(decode(&[1u8]), Err(MemoDecodeError::Invalid));
    }

    #[test]
    fn over_max_len_is_too_long_and_exactly_max_is_not() {
        assert_eq!(
            decode(&vec![1u8; MEMO_MAX_LEN + 1]),
            Err(MemoDecodeError::TooLong)
        );
        assert_eq!(
            decode(&vec![1u8; MEMO_MAX_LEN]),
            Err(MemoDecodeError::Invalid),
            "at the limit it is decoded, and fails on its contents"
        );
    }

    #[test]
    fn an_unknown_version_is_never_best_effort_parsed() {
        let mut bytes = encode(&memo(None, BalanceBucket::Cross));
        bytes[0] = 2;
        assert_eq!(decode(&bytes), Err(MemoDecodeError::Invalid));
    }

    #[test]
    fn trailing_bytes_are_rejected_not_ignored() {
        let mut bytes = encode(&memo(None, BalanceBucket::Cross));
        bytes.push(0x00);
        assert_eq!(decode(&bytes), Err(MemoDecodeError::Invalid));
    }

    #[test]
    fn an_unknown_balance_bucket_is_rejected() {
        let mut bytes = encode(&memo(None, BalanceBucket::Bank));
        *bytes.last_mut().expect("non-empty") = 0xFE;
        assert_eq!(decode(&bytes), Err(MemoDecodeError::Invalid));
    }

    /// A memo naming a market that does not exist still decodes, and the operator
    /// still credits it — the exchange then reverts the transfer and the uniqueness
    /// nonce is not consumed, so the reconciler retries it. Not forever: the retry
    /// lasts only while the nonce stays inside the replay window, after which the
    /// deposit is reported for manual recovery like any other stranded one.
    ///
    /// The alternative — rejecting here so the operator pays the depositor instead —
    /// would turn a loud, reversible failure into a silent, final one: the funds
    /// would land somewhere the depositor did not ask for, and no later fix could
    /// move them. Market existence is also only one of the exchange's conditions
    /// (the market must be operational, the asset USDC, the recipient's ledger in
    /// iso mode), all of them live per-account state, so screening for it here would
    /// not even remove the failure class it targets.
    #[test]
    fn an_unknown_market_id_is_left_to_the_exchange() {
        let m = memo(None, BalanceBucket::Iso(MarketId(u16::MAX)));
        assert_eq!(decode(&encode(&m)), Ok(m));
    }

    /// Paying the zero address would burn the deposit as surely as dropping it.
    #[test]
    fn the_zero_address_is_rejected() {
        let bytes = encode(&Memo::V1(MemoV1 {
            to_address: Address([0u8; 32]),
            to: TransferEndpoint {
                sub_account_index: None,
                balance: BalanceBucket::Bank,
            },
        }));
        assert_eq!(decode(&bytes), Err(MemoDecodeError::Invalid));
    }
}
