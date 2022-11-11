#![feature(prelude_import)]
//! The `core` module provides all the pure Rust Wormhole primitives.
//!
//! This crate provides chain-agnostic types from Wormhole for consumption in on-chain contracts
//! and within other chain-specific Wormhole Rust SDK's. It includes:
//!
//! - Constants containing known network data/addresses.
//! - Parsers for VAA's and Payloads.
//! - Data types for Wormhole primitives such as GuardianSets and signatures.
//! - Verification Primitives for securely checking payloads.
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde::{Deserialize, Serialize};
mod arraystring {
    //! A module for serializing/deserializing a `BString` as a fixed-width 32 byte array.
    use std::{convert::identity, fmt, iter::repeat};
    use bstr::BString;
    use serde::{
        de::{Error as DeError, SeqAccess, Visitor},
        ser::{Error as SerError, SerializeTuple},
        Deserializer, Serializer,
    };
    pub fn serialize<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        let v = value.as_ref();
        let l = v.len();
        if l > 32 {
            return Err(
                S::Error::custom(
                    ::core::fmt::Arguments::new_v1(
                        &["value is too large (", " bytes); max 32"],
                        &[::core::fmt::ArgumentV1::new_display(&l)],
                    ),
                ),
            );
        }
        let mut tup = serializer.serialize_tuple(32)?;
        for e in repeat(&0u8).take(32 - l).chain(v) {
            tup.serialize_element(e)?;
        }
        tup.end()
    }
    struct ArrayStringVisitor;
    impl<'de> Visitor<'de> for ArrayStringVisitor {
        type Value = BString;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an array of 32 bytes")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut buf = Vec::with_capacity(32);
            for i in 0..32 {
                let e = seq
                    .next_element()
                    .map(|e| e.ok_or_else(|| A::Error::invalid_length(i, &self)))
                    .and_then(identity)?;
                if e == 0 && buf.is_empty() {
                    continue;
                }
                buf.push(e);
            }
            Ok(BString::from(buf))
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BString, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(32, ArrayStringVisitor)
    }
}
mod chain {
    //! Provide Types and Data about Wormhole's supported chains.
    use std::{fmt, str::FromStr};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub enum Chain {
        /// In the wormhole wire format, 0 indicates that a message is for any destination chain, it is
        /// represented here as `Any`.
        Any,
        /// Chains
        Solana,
        Ethereum,
        Terra,
        Bsc,
        Polygon,
        Avalanche,
        Oasis,
        Algorand,
        Aurora,
        Fantom,
        Karura,
        Acala,
        Klaytn,
        Celo,
        Near,
        Moonbeam,
        Neon,
        Terra2,
        Injective,
        Osmosis,
        Sui,
        Aptos,
        Arbitrum,
        Optimism,
        Gnosis,
        Pythnet,
        Xpla,
        Ropsten,
        Wormchain,
        Unknown(u16),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Chain {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Chain::Any => ::core::fmt::Formatter::write_str(f, "Any"),
                Chain::Solana => ::core::fmt::Formatter::write_str(f, "Solana"),
                Chain::Ethereum => ::core::fmt::Formatter::write_str(f, "Ethereum"),
                Chain::Terra => ::core::fmt::Formatter::write_str(f, "Terra"),
                Chain::Bsc => ::core::fmt::Formatter::write_str(f, "Bsc"),
                Chain::Polygon => ::core::fmt::Formatter::write_str(f, "Polygon"),
                Chain::Avalanche => ::core::fmt::Formatter::write_str(f, "Avalanche"),
                Chain::Oasis => ::core::fmt::Formatter::write_str(f, "Oasis"),
                Chain::Algorand => ::core::fmt::Formatter::write_str(f, "Algorand"),
                Chain::Aurora => ::core::fmt::Formatter::write_str(f, "Aurora"),
                Chain::Fantom => ::core::fmt::Formatter::write_str(f, "Fantom"),
                Chain::Karura => ::core::fmt::Formatter::write_str(f, "Karura"),
                Chain::Acala => ::core::fmt::Formatter::write_str(f, "Acala"),
                Chain::Klaytn => ::core::fmt::Formatter::write_str(f, "Klaytn"),
                Chain::Celo => ::core::fmt::Formatter::write_str(f, "Celo"),
                Chain::Near => ::core::fmt::Formatter::write_str(f, "Near"),
                Chain::Moonbeam => ::core::fmt::Formatter::write_str(f, "Moonbeam"),
                Chain::Neon => ::core::fmt::Formatter::write_str(f, "Neon"),
                Chain::Terra2 => ::core::fmt::Formatter::write_str(f, "Terra2"),
                Chain::Injective => ::core::fmt::Formatter::write_str(f, "Injective"),
                Chain::Osmosis => ::core::fmt::Formatter::write_str(f, "Osmosis"),
                Chain::Sui => ::core::fmt::Formatter::write_str(f, "Sui"),
                Chain::Aptos => ::core::fmt::Formatter::write_str(f, "Aptos"),
                Chain::Arbitrum => ::core::fmt::Formatter::write_str(f, "Arbitrum"),
                Chain::Optimism => ::core::fmt::Formatter::write_str(f, "Optimism"),
                Chain::Gnosis => ::core::fmt::Formatter::write_str(f, "Gnosis"),
                Chain::Pythnet => ::core::fmt::Formatter::write_str(f, "Pythnet"),
                Chain::Xpla => ::core::fmt::Formatter::write_str(f, "Xpla"),
                Chain::Ropsten => ::core::fmt::Formatter::write_str(f, "Ropsten"),
                Chain::Wormchain => ::core::fmt::Formatter::write_str(f, "Wormchain"),
                Chain::Unknown(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Unknown",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Chain {
        #[inline]
        fn clone(&self) -> Chain {
            let _: ::core::clone::AssertParamIsClone<u16>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Chain {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Chain {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Chain {
        #[inline]
        fn eq(&self, other: &Chain) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Chain::Unknown(__self_0), Chain::Unknown(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Chain {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Chain {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u16>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Chain {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Chain,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match (self, other) {
                        (Chain::Unknown(__self_0), Chain::Unknown(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Chain {
        #[inline]
        fn cmp(&self, other: &Chain) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                ::core::cmp::Ordering::Equal => {
                    match (self, other) {
                        (Chain::Unknown(__self_0), Chain::Unknown(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Chain {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Chain::Unknown(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                _ => {}
            }
        }
    }
    impl fmt::Display for Chain {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Any => f.write_str("Any"),
                Self::Solana => f.write_str("Solana"),
                Self::Ethereum => f.write_str("Ethereum"),
                Self::Terra => f.write_str("Terra"),
                Self::Bsc => f.write_str("Bsc"),
                Self::Polygon => f.write_str("Polygon"),
                Self::Avalanche => f.write_str("Avalanche"),
                Self::Oasis => f.write_str("Oasis"),
                Self::Algorand => f.write_str("Algorand"),
                Self::Aurora => f.write_str("Aurora"),
                Self::Fantom => f.write_str("Fantom"),
                Self::Karura => f.write_str("Karura"),
                Self::Acala => f.write_str("Acala"),
                Self::Klaytn => f.write_str("Klaytn"),
                Self::Celo => f.write_str("Celo"),
                Self::Near => f.write_str("Near"),
                Self::Moonbeam => f.write_str("Moonbeam"),
                Self::Neon => f.write_str("Neon"),
                Self::Terra2 => f.write_str("Terra2"),
                Self::Injective => f.write_str("Injective"),
                Self::Osmosis => f.write_str("Osmosis"),
                Self::Sui => f.write_str("Sui"),
                Self::Aptos => f.write_str("Aptos"),
                Self::Arbitrum => f.write_str("Arbitrum"),
                Self::Optimism => f.write_str("Optimism"),
                Self::Gnosis => f.write_str("Gnosis"),
                Self::Pythnet => f.write_str("Pythnet"),
                Self::Xpla => f.write_str("Xpla"),
                Self::Ropsten => f.write_str("Ropsten"),
                Self::Wormchain => f.write_str("Wormchain"),
                Self::Unknown(v) => {
                    f
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["Unknown(", ")"],
                                &[::core::fmt::ArgumentV1::new_display(&v)],
                            ),
                        )
                }
            }
        }
    }
    impl FromStr for Chain {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Any" | "any" | "ANY" => Ok(Chain::Any),
                "Solana" | "solana" | "SOLANA" => Ok(Chain::Solana),
                "Ethereum" | "ethereum" | "ETHEREUM" => Ok(Chain::Ethereum),
                "Terra" | "terra" | "TERRA" => Ok(Chain::Terra),
                "Bsc" | "bsc" | "BSC" => Ok(Chain::Bsc),
                "Polygon" | "polygon" | "POLYGON" => Ok(Chain::Polygon),
                "Avalanche" | "avalanche" | "AVALANCHE" => Ok(Chain::Avalanche),
                "Oasis" | "oasis" | "OASIS" => Ok(Chain::Oasis),
                "Algorand" | "algorand" | "ALGORAND" => Ok(Chain::Algorand),
                "Aurora" | "aurora" | "AURORA" => Ok(Chain::Aurora),
                "Fantom" | "fantom" | "FANTOM" => Ok(Chain::Fantom),
                "Karura" | "karura" | "KARURA" => Ok(Chain::Karura),
                "Acala" | "acala" | "ACALA" => Ok(Chain::Acala),
                "Klaytn" | "klaytn" | "KLAYTN" => Ok(Chain::Klaytn),
                "Celo" | "celo" | "CELO" => Ok(Chain::Celo),
                "Near" | "near" | "NEAR" => Ok(Chain::Near),
                "Moonbeam" | "moonbeam" | "MOONBEAM" => Ok(Chain::Moonbeam),
                "Neon" | "neon" | "NEON" => Ok(Chain::Neon),
                "Terra2" | "terra2" | "TERRA2" => Ok(Chain::Terra2),
                "Injective" | "injective" | "INJECTIVE" => Ok(Chain::Injective),
                "Osmosis" | "osmosis" | "OSMOSIS" => Ok(Chain::Osmosis),
                "Sui" | "sui" | "SUI" => Ok(Chain::Sui),
                "Aptos" | "aptos" | "APTOS" => Ok(Chain::Aptos),
                "Arbitrum" | "arbitrum" | "ARBITRUM" => Ok(Chain::Arbitrum),
                "Optimism" | "optimism" | "OPTIMISM" => Ok(Chain::Optimism),
                "Gnosis" | "gnosis" | "GNOSIS" => Ok(Chain::Gnosis),
                "Pythnet" | "pythnet" | "PYTHNET" => Ok(Chain::Pythnet),
                "Xpla" | "xpla" | "XPLA" => Ok(Chain::Xpla),
                "Ropsten" | "ropsten" | "ROPSTEN" => Ok(Chain::Ropsten),
                "Wormchain" | "wormchain" | "WORMCHAIN" => Ok(Chain::Wormchain),
                _ => {
                    Err({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["invalid chain: "],
                                &[::core::fmt::ArgumentV1::new_display(&s)],
                            ),
                        );
                        res
                    })
                }
            }
        }
    }
    impl From<u16> for Chain {
        fn from(other: u16) -> Chain {
            match other {
                0 => Chain::Any,
                1 => Chain::Solana,
                2 => Chain::Ethereum,
                3 => Chain::Terra,
                4 => Chain::Bsc,
                5 => Chain::Polygon,
                6 => Chain::Avalanche,
                7 => Chain::Oasis,
                8 => Chain::Algorand,
                9 => Chain::Aurora,
                10 => Chain::Fantom,
                11 => Chain::Karura,
                12 => Chain::Acala,
                13 => Chain::Klaytn,
                14 => Chain::Celo,
                15 => Chain::Near,
                16 => Chain::Moonbeam,
                17 => Chain::Neon,
                18 => Chain::Terra2,
                19 => Chain::Injective,
                20 => Chain::Osmosis,
                21 => Chain::Sui,
                22 => Chain::Aptos,
                23 => Chain::Arbitrum,
                24 => Chain::Optimism,
                25 => Chain::Gnosis,
                26 => Chain::Pythnet,
                28 => Chain::Xpla,
                3104 => Chain::Wormchain,
                10001 => Chain::Ropsten,
                c => Chain::Unknown(c),
            }
        }
    }
    impl From<Chain> for u16 {
        fn from(other: Chain) -> u16 {
            match other {
                Chain::Any => 0,
                Chain::Solana => 1,
                Chain::Ethereum => 2,
                Chain::Terra => 3,
                Chain::Bsc => 4,
                Chain::Polygon => 5,
                Chain::Avalanche => 6,
                Chain::Oasis => 7,
                Chain::Algorand => 8,
                Chain::Aurora => 9,
                Chain::Fantom => 10,
                Chain::Karura => 11,
                Chain::Acala => 12,
                Chain::Klaytn => 13,
                Chain::Celo => 14,
                Chain::Near => 15,
                Chain::Moonbeam => 16,
                Chain::Neon => 17,
                Chain::Terra2 => 18,
                Chain::Injective => 19,
                Chain::Osmosis => 20,
                Chain::Sui => 21,
                Chain::Aptos => 22,
                Chain::Arbitrum => 23,
                Chain::Optimism => 24,
                Chain::Gnosis => 25,
                Chain::Pythnet => 26,
                Chain::Xpla => 28,
                Chain::Wormchain => 3104,
                Chain::Ropsten => 10001,
                Chain::Unknown(c) => c,
            }
        }
    }
    impl Default for Chain {
        fn default() -> Self {
            Self::Any
        }
    }
    impl Serialize for Chain {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u16((*self).into())
        }
    }
    impl<'de> Deserialize<'de> for Chain {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <u16 as Deserialize>::deserialize(deserializer).map(Self::from)
        }
    }
}
mod serde_array {
    use std::{fmt, mem::MaybeUninit};
    use serde::{
        de::{Error, SeqAccess, Visitor},
        ser::SerializeTuple, Deserializer, Serializer,
    };
    pub fn serialize<const N: usize, S>(
        value: &[u8; N],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(N)?;
        for v in value {
            seq.serialize_element(v)?;
        }
        seq.end()
    }
    struct ArrayVisitor<const N: usize>;
    impl<'de, const N: usize> Visitor<'de> for ArrayVisitor<N> {
        type Value = [u8; N];
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["an array of length "],
                        &[::core::fmt::ArgumentV1::new_display(&N)],
                    ),
                )
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut buf = MaybeUninit::<[u8; N]>::uninit();
            let ptr = buf.as_mut_ptr() as *mut u8;
            let mut pos = 0;
            while pos < N {
                let v = seq
                    .next_element()
                    .and_then(|v| v.ok_or_else(|| Error::invalid_length(pos, &self)))?;
                unsafe { ptr.add(pos).write(v) };
                pos += 1;
            }
            if pos == N {
                Ok(unsafe { buf.assume_init() })
            } else {
                Err(Error::invalid_length(pos, &self))
            }
        }
    }
    pub fn deserialize<'de, const N: usize, D>(
        deserializer: D,
    ) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(N, ArrayVisitor)
    }
}
pub mod token {
    //! Parsers for Token bridge VAAs.
    //!
    //! Token bridging relies on VAA's that indicate custody/lockup/burn events in order to maintain
    //! token parity between multiple chains. These parsers can be used to read these VAAs. It also
    //! defines the Governance actions that this module supports, namely contract upgrades and chain
    //! registrations.
    use bstr::BString;
    use serde::{Deserialize, Serialize};
    use crate::{Address, Amount, Chain};
    pub enum Message {
        #[serde(rename = "1")]
        Transfer {
            amount: Amount,
            token_address: Address,
            token_chain: Chain,
            recipient: Address,
            recipient_chain: Chain,
            fee: Amount,
        },
        #[serde(rename = "2")]
        AssetMeta {
            token_address: Address,
            token_chain: Chain,
            decimals: u8,
            #[serde(with = "crate::arraystring")]
            symbol: BString,
            #[serde(with = "crate::arraystring")]
            name: BString,
        },
        #[serde(rename = "3")]
        TransferWithPayload {
            amount: Amount,
            token_address: Address,
            token_chain: Chain,
            recipient: Address,
            recipient_chain: Chain,
            sender_address: Address,
        },
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Message {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Message::Transfer {
                        ref amount,
                        ref token_address,
                        ref token_chain,
                        ref recipient,
                        ref recipient_chain,
                        ref fee,
                    } => {
                        let mut __serde_state = match _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "Message",
                            0u32,
                            "1",
                            0 + 1 + 1 + 1 + 1 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "amount",
                            amount,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "token_address",
                            token_address,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "token_chain",
                            token_chain,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "recipient",
                            recipient,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "recipient_chain",
                            recipient_chain,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "fee",
                            fee,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    Message::AssetMeta {
                        ref token_address,
                        ref token_chain,
                        ref decimals,
                        ref symbol,
                        ref name,
                    } => {
                        let mut __serde_state = match _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "Message",
                            1u32,
                            "2",
                            0 + 1 + 1 + 1 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "token_address",
                            token_address,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "token_chain",
                            token_chain,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "decimals",
                            decimals,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "symbol",
                            {
                                struct __SerializeWith<'__a> {
                                    values: (&'__a BString,),
                                    phantom: _serde::__private::PhantomData<Message>,
                                }
                                impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                    fn serialize<__S>(
                                        &self,
                                        __s: __S,
                                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                    where
                                        __S: _serde::Serializer,
                                    {
                                        crate::arraystring::serialize(self.values.0, __s)
                                    }
                                }
                                &__SerializeWith {
                                    values: (symbol,),
                                    phantom: _serde::__private::PhantomData::<Message>,
                                }
                            },
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "name",
                            {
                                struct __SerializeWith<'__a> {
                                    values: (&'__a BString,),
                                    phantom: _serde::__private::PhantomData<Message>,
                                }
                                impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                    fn serialize<__S>(
                                        &self,
                                        __s: __S,
                                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                    where
                                        __S: _serde::Serializer,
                                    {
                                        crate::arraystring::serialize(self.values.0, __s)
                                    }
                                }
                                &__SerializeWith {
                                    values: (name,),
                                    phantom: _serde::__private::PhantomData::<Message>,
                                }
                            },
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    Message::TransferWithPayload {
                        ref amount,
                        ref token_address,
                        ref token_chain,
                        ref recipient,
                        ref recipient_chain,
                        ref sender_address,
                    } => {
                        let mut __serde_state = match _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "Message",
                            2u32,
                            "3",
                            0 + 1 + 1 + 1 + 1 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "amount",
                            amount,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "token_address",
                            token_address,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "token_chain",
                            token_chain,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "recipient",
                            recipient,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "recipient_chain",
                            recipient_chain,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "sender_address",
                            sender_address,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Message {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 3",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "1" => _serde::__private::Ok(__Field::__field0),
                            "2" => _serde::__private::Ok(__Field::__field1),
                            "3" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"1" => _serde::__private::Ok(__Field::__field0),
                            b"2" => _serde::__private::Ok(__Field::__field1),
                            b"3" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Message>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Message;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum Message",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __field3,
                                    __field4,
                                    __field5,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            3u64 => _serde::__private::Ok(__Field::__field3),
                                            4u64 => _serde::__private::Ok(__Field::__field4),
                                            5u64 => _serde::__private::Ok(__Field::__field5),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "amount" => _serde::__private::Ok(__Field::__field0),
                                            "token_address" => _serde::__private::Ok(__Field::__field1),
                                            "token_chain" => _serde::__private::Ok(__Field::__field2),
                                            "recipient" => _serde::__private::Ok(__Field::__field3),
                                            "recipient_chain" => {
                                                _serde::__private::Ok(__Field::__field4)
                                            }
                                            "fee" => _serde::__private::Ok(__Field::__field5),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"amount" => _serde::__private::Ok(__Field::__field0),
                                            b"token_address" => _serde::__private::Ok(__Field::__field1),
                                            b"token_chain" => _serde::__private::Ok(__Field::__field2),
                                            b"recipient" => _serde::__private::Ok(__Field::__field3),
                                            b"recipient_chain" => {
                                                _serde::__private::Ok(__Field::__field4)
                                            }
                                            b"fee" => _serde::__private::Ok(__Field::__field5),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<Message>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Message;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant Message::Transfer",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                                            Amount,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant Message::Transfer with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant Message::Transfer with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                                            Chain,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant Message::Transfer with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        3usize,
                                                        &"struct variant Message::Transfer with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                                            Chain,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        4usize,
                                                        &"struct variant Message::Transfer with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                                            Amount,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        5usize,
                                                        &"struct variant Message::Transfer with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(Message::Transfer {
                                            amount: __field0,
                                            token_address: __field1,
                                            token_chain: __field2,
                                            recipient: __field3,
                                            recipient_chain: __field4,
                                            fee: __field5,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Amount> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Address> = _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<Chain> = _serde::__private::None;
                                        let mut __field3: _serde::__private::Option<Address> = _serde::__private::None;
                                        let mut __field4: _serde::__private::Option<Chain> = _serde::__private::None;
                                        let mut __field5: _serde::__private::Option<Amount> = _serde::__private::None;
                                        while let _serde::__private::Some(__key)
                                            = match _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Amount,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "token_address",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "token_chain",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Chain,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field3 => {
                                                    if _serde::__private::Option::is_some(&__field3) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "recipient",
                                                            ),
                                                        );
                                                    }
                                                    __field3 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field4 => {
                                                    if _serde::__private::Option::is_some(&__field4) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "recipient_chain",
                                                            ),
                                                        );
                                                    }
                                                    __field4 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Chain,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field5 => {
                                                    if _serde::__private::Option::is_some(&__field5) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("fee"),
                                                        );
                                                    }
                                                    __field5 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Amount,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("amount") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field(
                                                    "token_address",
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("token_chain") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field3 = match __field3 {
                                            _serde::__private::Some(__field3) => __field3,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("recipient") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field4 = match __field4 {
                                            _serde::__private::Some(__field4) => __field4,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field(
                                                    "recipient_chain",
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field5 = match __field5 {
                                            _serde::__private::Some(__field5) => __field5,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("fee") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        _serde::__private::Ok(Message::Transfer {
                                            amount: __field0,
                                            token_address: __field1,
                                            token_chain: __field2,
                                            recipient: __field3,
                                            recipient_chain: __field4,
                                            fee: __field5,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] = &[
                                    "amount",
                                    "token_address",
                                    "token_chain",
                                    "recipient",
                                    "recipient_chain",
                                    "fee",
                                ];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<Message>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field1, __variant) => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __field3,
                                    __field4,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            3u64 => _serde::__private::Ok(__Field::__field3),
                                            4u64 => _serde::__private::Ok(__Field::__field4),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "token_address" => _serde::__private::Ok(__Field::__field0),
                                            "token_chain" => _serde::__private::Ok(__Field::__field1),
                                            "decimals" => _serde::__private::Ok(__Field::__field2),
                                            "symbol" => _serde::__private::Ok(__Field::__field3),
                                            "name" => _serde::__private::Ok(__Field::__field4),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"token_address" => _serde::__private::Ok(__Field::__field0),
                                            b"token_chain" => _serde::__private::Ok(__Field::__field1),
                                            b"decimals" => _serde::__private::Ok(__Field::__field2),
                                            b"symbol" => _serde::__private::Ok(__Field::__field3),
                                            b"name" => _serde::__private::Ok(__Field::__field4),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<Message>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Message;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant Message::AssetMeta",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant Message::AssetMeta with 5 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                                            Chain,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant Message::AssetMeta with 5 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                                            u8,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant Message::AssetMeta with 5 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field3 = match {
                                            struct __DeserializeWith<'de> {
                                                value: BString,
                                                phantom: _serde::__private::PhantomData<Message>,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private::Ok(__DeserializeWith {
                                                        value: match crate::arraystring::deserialize(
                                                            __deserializer,
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                        phantom: _serde::__private::PhantomData,
                                                        lifetime: _serde::__private::PhantomData,
                                                    })
                                                }
                                            }
                                            _serde::__private::Option::map(
                                                match _serde::de::SeqAccess::next_element::<
                                                    __DeserializeWith<'de>,
                                                >(&mut __seq) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                                |__wrap| __wrap.value,
                                            )
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        3usize,
                                                        &"struct variant Message::AssetMeta with 5 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field4 = match {
                                            struct __DeserializeWith<'de> {
                                                value: BString,
                                                phantom: _serde::__private::PhantomData<Message>,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private::Ok(__DeserializeWith {
                                                        value: match crate::arraystring::deserialize(
                                                            __deserializer,
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                        phantom: _serde::__private::PhantomData,
                                                        lifetime: _serde::__private::PhantomData,
                                                    })
                                                }
                                            }
                                            _serde::__private::Option::map(
                                                match _serde::de::SeqAccess::next_element::<
                                                    __DeserializeWith<'de>,
                                                >(&mut __seq) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                                |__wrap| __wrap.value,
                                            )
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        4usize,
                                                        &"struct variant Message::AssetMeta with 5 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(Message::AssetMeta {
                                            token_address: __field0,
                                            token_chain: __field1,
                                            decimals: __field2,
                                            symbol: __field3,
                                            name: __field4,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Address> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Chain> = _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<u8> = _serde::__private::None;
                                        let mut __field3: _serde::__private::Option<BString> = _serde::__private::None;
                                        let mut __field4: _serde::__private::Option<BString> = _serde::__private::None;
                                        while let _serde::__private::Some(__key)
                                            = match _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "token_address",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "token_chain",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Chain,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "decimals",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field3 => {
                                                    if _serde::__private::Option::is_some(&__field3) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("symbol"),
                                                        );
                                                    }
                                                    __field3 = _serde::__private::Some({
                                                        struct __DeserializeWith<'de> {
                                                            value: BString,
                                                            phantom: _serde::__private::PhantomData<Message>,
                                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                                        }
                                                        impl<'de> _serde::Deserialize<'de>
                                                        for __DeserializeWith<'de> {
                                                            fn deserialize<__D>(
                                                                __deserializer: __D,
                                                            ) -> _serde::__private::Result<Self, __D::Error>
                                                            where
                                                                __D: _serde::Deserializer<'de>,
                                                            {
                                                                _serde::__private::Ok(__DeserializeWith {
                                                                    value: match crate::arraystring::deserialize(
                                                                        __deserializer,
                                                                    ) {
                                                                        _serde::__private::Ok(__val) => __val,
                                                                        _serde::__private::Err(__err) => {
                                                                            return _serde::__private::Err(__err);
                                                                        }
                                                                    },
                                                                    phantom: _serde::__private::PhantomData,
                                                                    lifetime: _serde::__private::PhantomData,
                                                                })
                                                            }
                                                        }
                                                        match _serde::de::MapAccess::next_value::<
                                                            __DeserializeWith<'de>,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        }
                                                    });
                                                }
                                                __Field::__field4 => {
                                                    if _serde::__private::Option::is_some(&__field4) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                        );
                                                    }
                                                    __field4 = _serde::__private::Some({
                                                        struct __DeserializeWith<'de> {
                                                            value: BString,
                                                            phantom: _serde::__private::PhantomData<Message>,
                                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                                        }
                                                        impl<'de> _serde::Deserialize<'de>
                                                        for __DeserializeWith<'de> {
                                                            fn deserialize<__D>(
                                                                __deserializer: __D,
                                                            ) -> _serde::__private::Result<Self, __D::Error>
                                                            where
                                                                __D: _serde::Deserializer<'de>,
                                                            {
                                                                _serde::__private::Ok(__DeserializeWith {
                                                                    value: match crate::arraystring::deserialize(
                                                                        __deserializer,
                                                                    ) {
                                                                        _serde::__private::Ok(__val) => __val,
                                                                        _serde::__private::Err(__err) => {
                                                                            return _serde::__private::Err(__err);
                                                                        }
                                                                    },
                                                                    phantom: _serde::__private::PhantomData,
                                                                    lifetime: _serde::__private::PhantomData,
                                                                })
                                                            }
                                                        }
                                                        match _serde::de::MapAccess::next_value::<
                                                            __DeserializeWith<'de>,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        }
                                                    });
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field(
                                                    "token_address",
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("token_chain") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("decimals") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field3 = match __field3 {
                                            _serde::__private::Some(__field3) => __field3,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::missing_field("symbol"),
                                                );
                                            }
                                        };
                                        let __field4 = match __field4 {
                                            _serde::__private::Some(__field4) => __field4,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::missing_field("name"),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(Message::AssetMeta {
                                            token_address: __field0,
                                            token_chain: __field1,
                                            decimals: __field2,
                                            symbol: __field3,
                                            name: __field4,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] = &[
                                    "token_address",
                                    "token_chain",
                                    "decimals",
                                    "symbol",
                                    "name",
                                ];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<Message>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field2, __variant) => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __field3,
                                    __field4,
                                    __field5,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            3u64 => _serde::__private::Ok(__Field::__field3),
                                            4u64 => _serde::__private::Ok(__Field::__field4),
                                            5u64 => _serde::__private::Ok(__Field::__field5),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "amount" => _serde::__private::Ok(__Field::__field0),
                                            "token_address" => _serde::__private::Ok(__Field::__field1),
                                            "token_chain" => _serde::__private::Ok(__Field::__field2),
                                            "recipient" => _serde::__private::Ok(__Field::__field3),
                                            "recipient_chain" => {
                                                _serde::__private::Ok(__Field::__field4)
                                            }
                                            "sender_address" => _serde::__private::Ok(__Field::__field5),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"amount" => _serde::__private::Ok(__Field::__field0),
                                            b"token_address" => _serde::__private::Ok(__Field::__field1),
                                            b"token_chain" => _serde::__private::Ok(__Field::__field2),
                                            b"recipient" => _serde::__private::Ok(__Field::__field3),
                                            b"recipient_chain" => {
                                                _serde::__private::Ok(__Field::__field4)
                                            }
                                            b"sender_address" => {
                                                _serde::__private::Ok(__Field::__field5)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<Message>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Message;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant Message::TransferWithPayload",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                                            Amount,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant Message::TransferWithPayload with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant Message::TransferWithPayload with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                                            Chain,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant Message::TransferWithPayload with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        3usize,
                                                        &"struct variant Message::TransferWithPayload with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                                            Chain,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        4usize,
                                                        &"struct variant Message::TransferWithPayload with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        5usize,
                                                        &"struct variant Message::TransferWithPayload with 6 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(Message::TransferWithPayload {
                                            amount: __field0,
                                            token_address: __field1,
                                            token_chain: __field2,
                                            recipient: __field3,
                                            recipient_chain: __field4,
                                            sender_address: __field5,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Amount> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Address> = _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<Chain> = _serde::__private::None;
                                        let mut __field3: _serde::__private::Option<Address> = _serde::__private::None;
                                        let mut __field4: _serde::__private::Option<Chain> = _serde::__private::None;
                                        let mut __field5: _serde::__private::Option<Address> = _serde::__private::None;
                                        while let _serde::__private::Some(__key)
                                            = match _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Amount,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "token_address",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "token_chain",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Chain,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field3 => {
                                                    if _serde::__private::Option::is_some(&__field3) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "recipient",
                                                            ),
                                                        );
                                                    }
                                                    __field3 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field4 => {
                                                    if _serde::__private::Option::is_some(&__field4) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "recipient_chain",
                                                            ),
                                                        );
                                                    }
                                                    __field4 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Chain,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field5 => {
                                                    if _serde::__private::Option::is_some(&__field5) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "sender_address",
                                                            ),
                                                        );
                                                    }
                                                    __field5 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("amount") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field(
                                                    "token_address",
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("token_chain") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field3 = match __field3 {
                                            _serde::__private::Some(__field3) => __field3,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("recipient") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field4 = match __field4 {
                                            _serde::__private::Some(__field4) => __field4,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field(
                                                    "recipient_chain",
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field5 = match __field5 {
                                            _serde::__private::Some(__field5) => __field5,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field(
                                                    "sender_address",
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        _serde::__private::Ok(Message::TransferWithPayload {
                                            amount: __field0,
                                            token_address: __field1,
                                            token_chain: __field2,
                                            recipient: __field3,
                                            recipient_chain: __field4,
                                            sender_address: __field5,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] = &[
                                    "amount",
                                    "token_address",
                                    "token_chain",
                                    "recipient",
                                    "recipient_chain",
                                    "sender_address",
                                ];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<Message>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["1", "2", "3"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Message",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Message>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Message {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Message::Transfer {
                    amount: __self_0,
                    token_address: __self_1,
                    token_chain: __self_2,
                    recipient: __self_3,
                    recipient_chain: __self_4,
                    fee: __self_5,
                } => {
                    let names: &'static _ = &[
                        "amount",
                        "token_address",
                        "token_chain",
                        "recipient",
                        "recipient_chain",
                        "fee",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &__self_0,
                        &__self_1,
                        &__self_2,
                        &__self_3,
                        &__self_4,
                        &__self_5,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "Transfer",
                        names,
                        values,
                    )
                }
                Message::AssetMeta {
                    token_address: __self_0,
                    token_chain: __self_1,
                    decimals: __self_2,
                    symbol: __self_3,
                    name: __self_4,
                } => {
                    ::core::fmt::Formatter::debug_struct_field5_finish(
                        f,
                        "AssetMeta",
                        "token_address",
                        &__self_0,
                        "token_chain",
                        &__self_1,
                        "decimals",
                        &__self_2,
                        "symbol",
                        &__self_3,
                        "name",
                        &__self_4,
                    )
                }
                Message::TransferWithPayload {
                    amount: __self_0,
                    token_address: __self_1,
                    token_chain: __self_2,
                    recipient: __self_3,
                    recipient_chain: __self_4,
                    sender_address: __self_5,
                } => {
                    let names: &'static _ = &[
                        "amount",
                        "token_address",
                        "token_chain",
                        "recipient",
                        "recipient_chain",
                        "sender_address",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &__self_0,
                        &__self_1,
                        &__self_2,
                        &__self_3,
                        &__self_4,
                        &__self_5,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "TransferWithPayload",
                        names,
                        values,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Message {
        #[inline]
        fn clone(&self) -> Message {
            match self {
                Message::Transfer {
                    amount: __self_0,
                    token_address: __self_1,
                    token_chain: __self_2,
                    recipient: __self_3,
                    recipient_chain: __self_4,
                    fee: __self_5,
                } => {
                    Message::Transfer {
                        amount: ::core::clone::Clone::clone(__self_0),
                        token_address: ::core::clone::Clone::clone(__self_1),
                        token_chain: ::core::clone::Clone::clone(__self_2),
                        recipient: ::core::clone::Clone::clone(__self_3),
                        recipient_chain: ::core::clone::Clone::clone(__self_4),
                        fee: ::core::clone::Clone::clone(__self_5),
                    }
                }
                Message::AssetMeta {
                    token_address: __self_0,
                    token_chain: __self_1,
                    decimals: __self_2,
                    symbol: __self_3,
                    name: __self_4,
                } => {
                    Message::AssetMeta {
                        token_address: ::core::clone::Clone::clone(__self_0),
                        token_chain: ::core::clone::Clone::clone(__self_1),
                        decimals: ::core::clone::Clone::clone(__self_2),
                        symbol: ::core::clone::Clone::clone(__self_3),
                        name: ::core::clone::Clone::clone(__self_4),
                    }
                }
                Message::TransferWithPayload {
                    amount: __self_0,
                    token_address: __self_1,
                    token_chain: __self_2,
                    recipient: __self_3,
                    recipient_chain: __self_4,
                    sender_address: __self_5,
                } => {
                    Message::TransferWithPayload {
                        amount: ::core::clone::Clone::clone(__self_0),
                        token_address: ::core::clone::Clone::clone(__self_1),
                        token_chain: ::core::clone::Clone::clone(__self_2),
                        recipient: ::core::clone::Clone::clone(__self_3),
                        recipient_chain: ::core::clone::Clone::clone(__self_4),
                        sender_address: ::core::clone::Clone::clone(__self_5),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Message {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Message {
        #[inline]
        fn eq(&self, other: &Message) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        Message::Transfer {
                            amount: __self_0,
                            token_address: __self_1,
                            token_chain: __self_2,
                            recipient: __self_3,
                            recipient_chain: __self_4,
                            fee: __self_5,
                        },
                        Message::Transfer {
                            amount: __arg1_0,
                            token_address: __arg1_1,
                            token_chain: __arg1_2,
                            recipient: __arg1_3,
                            recipient_chain: __arg1_4,
                            fee: __arg1_5,
                        },
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2 && *__self_3 == *__arg1_3
                            && *__self_4 == *__arg1_4 && *__self_5 == *__arg1_5
                    }
                    (
                        Message::AssetMeta {
                            token_address: __self_0,
                            token_chain: __self_1,
                            decimals: __self_2,
                            symbol: __self_3,
                            name: __self_4,
                        },
                        Message::AssetMeta {
                            token_address: __arg1_0,
                            token_chain: __arg1_1,
                            decimals: __arg1_2,
                            symbol: __arg1_3,
                            name: __arg1_4,
                        },
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2 && *__self_3 == *__arg1_3
                            && *__self_4 == *__arg1_4
                    }
                    (
                        Message::TransferWithPayload {
                            amount: __self_0,
                            token_address: __self_1,
                            token_chain: __self_2,
                            recipient: __self_3,
                            recipient_chain: __self_4,
                            sender_address: __self_5,
                        },
                        Message::TransferWithPayload {
                            amount: __arg1_0,
                            token_address: __arg1_1,
                            token_chain: __arg1_2,
                            recipient: __arg1_3,
                            recipient_chain: __arg1_4,
                            sender_address: __arg1_5,
                        },
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2 && *__self_3 == *__arg1_3
                            && *__self_4 == *__arg1_4 && *__self_5 == *__arg1_5
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Message {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Message {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Amount>;
            let _: ::core::cmp::AssertParamIsEq<Address>;
            let _: ::core::cmp::AssertParamIsEq<Chain>;
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<BString>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Message {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Message,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match (self, other) {
                        (
                            Message::Transfer {
                                amount: __self_0,
                                token_address: __self_1,
                                token_chain: __self_2,
                                recipient: __self_3,
                                recipient_chain: __self_4,
                                fee: __self_5,
                            },
                            Message::Transfer {
                                amount: __arg1_0,
                                token_address: __arg1_1,
                                token_chain: __arg1_2,
                                recipient: __arg1_3,
                                recipient_chain: __arg1_4,
                                fee: __arg1_5,
                            },
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                __self_0,
                                __arg1_0,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        __self_1,
                                        __arg1_1,
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => {
                                            match ::core::cmp::PartialOrd::partial_cmp(
                                                __self_2,
                                                __arg1_2,
                                            ) {
                                                ::core::option::Option::Some(
                                                    ::core::cmp::Ordering::Equal,
                                                ) => {
                                                    match ::core::cmp::PartialOrd::partial_cmp(
                                                        __self_3,
                                                        __arg1_3,
                                                    ) {
                                                        ::core::option::Option::Some(
                                                            ::core::cmp::Ordering::Equal,
                                                        ) => {
                                                            match ::core::cmp::PartialOrd::partial_cmp(
                                                                __self_4,
                                                                __arg1_4,
                                                            ) {
                                                                ::core::option::Option::Some(
                                                                    ::core::cmp::Ordering::Equal,
                                                                ) => {
                                                                    ::core::cmp::PartialOrd::partial_cmp(__self_5, __arg1_5)
                                                                }
                                                                cmp => cmp,
                                                            }
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        (
                            Message::AssetMeta {
                                token_address: __self_0,
                                token_chain: __self_1,
                                decimals: __self_2,
                                symbol: __self_3,
                                name: __self_4,
                            },
                            Message::AssetMeta {
                                token_address: __arg1_0,
                                token_chain: __arg1_1,
                                decimals: __arg1_2,
                                symbol: __arg1_3,
                                name: __arg1_4,
                            },
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                __self_0,
                                __arg1_0,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        __self_1,
                                        __arg1_1,
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => {
                                            match ::core::cmp::PartialOrd::partial_cmp(
                                                __self_2,
                                                __arg1_2,
                                            ) {
                                                ::core::option::Option::Some(
                                                    ::core::cmp::Ordering::Equal,
                                                ) => {
                                                    match ::core::cmp::PartialOrd::partial_cmp(
                                                        __self_3,
                                                        __arg1_3,
                                                    ) {
                                                        ::core::option::Option::Some(
                                                            ::core::cmp::Ordering::Equal,
                                                        ) => {
                                                            ::core::cmp::PartialOrd::partial_cmp(__self_4, __arg1_4)
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        (
                            Message::TransferWithPayload {
                                amount: __self_0,
                                token_address: __self_1,
                                token_chain: __self_2,
                                recipient: __self_3,
                                recipient_chain: __self_4,
                                sender_address: __self_5,
                            },
                            Message::TransferWithPayload {
                                amount: __arg1_0,
                                token_address: __arg1_1,
                                token_chain: __arg1_2,
                                recipient: __arg1_3,
                                recipient_chain: __arg1_4,
                                sender_address: __arg1_5,
                            },
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                __self_0,
                                __arg1_0,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        __self_1,
                                        __arg1_1,
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => {
                                            match ::core::cmp::PartialOrd::partial_cmp(
                                                __self_2,
                                                __arg1_2,
                                            ) {
                                                ::core::option::Option::Some(
                                                    ::core::cmp::Ordering::Equal,
                                                ) => {
                                                    match ::core::cmp::PartialOrd::partial_cmp(
                                                        __self_3,
                                                        __arg1_3,
                                                    ) {
                                                        ::core::option::Option::Some(
                                                            ::core::cmp::Ordering::Equal,
                                                        ) => {
                                                            match ::core::cmp::PartialOrd::partial_cmp(
                                                                __self_4,
                                                                __arg1_4,
                                                            ) {
                                                                ::core::option::Option::Some(
                                                                    ::core::cmp::Ordering::Equal,
                                                                ) => {
                                                                    ::core::cmp::PartialOrd::partial_cmp(__self_5, __arg1_5)
                                                                }
                                                                cmp => cmp,
                                                            }
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Message {
        #[inline]
        fn cmp(&self, other: &Message) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                ::core::cmp::Ordering::Equal => {
                    match (self, other) {
                        (
                            Message::Transfer {
                                amount: __self_0,
                                token_address: __self_1,
                                token_chain: __self_2,
                                recipient: __self_3,
                                recipient_chain: __self_4,
                                fee: __self_5,
                            },
                            Message::Transfer {
                                amount: __arg1_0,
                                token_address: __arg1_1,
                                token_chain: __arg1_2,
                                recipient: __arg1_3,
                                recipient_chain: __arg1_4,
                                fee: __arg1_5,
                            },
                        ) => {
                            match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(__self_1, __arg1_1) {
                                        ::core::cmp::Ordering::Equal => {
                                            match ::core::cmp::Ord::cmp(__self_2, __arg1_2) {
                                                ::core::cmp::Ordering::Equal => {
                                                    match ::core::cmp::Ord::cmp(__self_3, __arg1_3) {
                                                        ::core::cmp::Ordering::Equal => {
                                                            match ::core::cmp::Ord::cmp(__self_4, __arg1_4) {
                                                                ::core::cmp::Ordering::Equal => {
                                                                    ::core::cmp::Ord::cmp(__self_5, __arg1_5)
                                                                }
                                                                cmp => cmp,
                                                            }
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        (
                            Message::AssetMeta {
                                token_address: __self_0,
                                token_chain: __self_1,
                                decimals: __self_2,
                                symbol: __self_3,
                                name: __self_4,
                            },
                            Message::AssetMeta {
                                token_address: __arg1_0,
                                token_chain: __arg1_1,
                                decimals: __arg1_2,
                                symbol: __arg1_3,
                                name: __arg1_4,
                            },
                        ) => {
                            match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(__self_1, __arg1_1) {
                                        ::core::cmp::Ordering::Equal => {
                                            match ::core::cmp::Ord::cmp(__self_2, __arg1_2) {
                                                ::core::cmp::Ordering::Equal => {
                                                    match ::core::cmp::Ord::cmp(__self_3, __arg1_3) {
                                                        ::core::cmp::Ordering::Equal => {
                                                            ::core::cmp::Ord::cmp(__self_4, __arg1_4)
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        (
                            Message::TransferWithPayload {
                                amount: __self_0,
                                token_address: __self_1,
                                token_chain: __self_2,
                                recipient: __self_3,
                                recipient_chain: __self_4,
                                sender_address: __self_5,
                            },
                            Message::TransferWithPayload {
                                amount: __arg1_0,
                                token_address: __arg1_1,
                                token_chain: __arg1_2,
                                recipient: __arg1_3,
                                recipient_chain: __arg1_4,
                                sender_address: __arg1_5,
                            },
                        ) => {
                            match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(__self_1, __arg1_1) {
                                        ::core::cmp::Ordering::Equal => {
                                            match ::core::cmp::Ord::cmp(__self_2, __arg1_2) {
                                                ::core::cmp::Ordering::Equal => {
                                                    match ::core::cmp::Ord::cmp(__self_3, __arg1_3) {
                                                        ::core::cmp::Ordering::Equal => {
                                                            match ::core::cmp::Ord::cmp(__self_4, __arg1_4) {
                                                                ::core::cmp::Ordering::Equal => {
                                                                    ::core::cmp::Ord::cmp(__self_5, __arg1_5)
                                                                }
                                                                cmp => cmp,
                                                            }
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Message {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Message::Transfer {
                    amount: __self_0,
                    token_address: __self_1,
                    token_chain: __self_2,
                    recipient: __self_3,
                    recipient_chain: __self_4,
                    fee: __self_5,
                } => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state);
                    ::core::hash::Hash::hash(__self_2, state);
                    ::core::hash::Hash::hash(__self_3, state);
                    ::core::hash::Hash::hash(__self_4, state);
                    ::core::hash::Hash::hash(__self_5, state)
                }
                Message::AssetMeta {
                    token_address: __self_0,
                    token_chain: __self_1,
                    decimals: __self_2,
                    symbol: __self_3,
                    name: __self_4,
                } => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state);
                    ::core::hash::Hash::hash(__self_2, state);
                    ::core::hash::Hash::hash(__self_3, state);
                    ::core::hash::Hash::hash(__self_4, state)
                }
                Message::TransferWithPayload {
                    amount: __self_0,
                    token_address: __self_1,
                    token_chain: __self_2,
                    recipient: __self_3,
                    recipient_chain: __self_4,
                    sender_address: __self_5,
                } => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state);
                    ::core::hash::Hash::hash(__self_2, state);
                    ::core::hash::Hash::hash(__self_3, state);
                    ::core::hash::Hash::hash(__self_4, state);
                    ::core::hash::Hash::hash(__self_5, state)
                }
            }
        }
    }
    pub enum Action {
        ContractUpgrade { new_contract: Address },
        RegisterChain { chain: Chain, emitter_address: Address },
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Action {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Action::ContractUpgrade { ref new_contract } => {
                        let mut __serde_state = match _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "Action",
                            0u32,
                            "ContractUpgrade",
                            0 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "new_contract",
                            new_contract,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    Action::RegisterChain { ref chain, ref emitter_address } => {
                        let mut __serde_state = match _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "Action",
                            1u32,
                            "RegisterChain",
                            0 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "chain",
                            chain,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "emitter_address",
                            emitter_address,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Action {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "ContractUpgrade" => _serde::__private::Ok(__Field::__field0),
                            "RegisterChain" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"ContractUpgrade" => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            b"RegisterChain" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Action>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Action;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum Action",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "new_contract" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"new_contract" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<Action>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Action;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant Action::ContractUpgrade",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant Action::ContractUpgrade with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(Action::ContractUpgrade {
                                            new_contract: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Address> = _serde::__private::None;
                                        while let _serde::__private::Some(__key)
                                            = match _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "new_contract",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("new_contract") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        _serde::__private::Ok(Action::ContractUpgrade {
                                            new_contract: __field0,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] = &["new_contract"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<Action>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field1, __variant) => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "chain" => _serde::__private::Ok(__Field::__field0),
                                            "emitter_address" => {
                                                _serde::__private::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"chain" => _serde::__private::Ok(__Field::__field0),
                                            b"emitter_address" => {
                                                _serde::__private::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<Action>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Action;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant Action::RegisterChain",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                                            Chain,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant Action::RegisterChain with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                                            Address,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant Action::RegisterChain with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(Action::RegisterChain {
                                            chain: __field0,
                                            emitter_address: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Chain> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Address> = _serde::__private::None;
                                        while let _serde::__private::Some(__key)
                                            = match _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("chain"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Chain,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "emitter_address",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Address,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("chain") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field(
                                                    "emitter_address",
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        _serde::__private::Ok(Action::RegisterChain {
                                            chain: __field0,
                                            emitter_address: __field1,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] = &[
                                    "chain",
                                    "emitter_address",
                                ];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<Action>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "ContractUpgrade",
                    "RegisterChain",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Action",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Action>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Action {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Action::ContractUpgrade { new_contract: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ContractUpgrade",
                        "new_contract",
                        &__self_0,
                    )
                }
                Action::RegisterChain { chain: __self_0, emitter_address: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "RegisterChain",
                        "chain",
                        &__self_0,
                        "emitter_address",
                        &__self_1,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Action {
        #[inline]
        fn clone(&self) -> Action {
            match self {
                Action::ContractUpgrade { new_contract: __self_0 } => {
                    Action::ContractUpgrade {
                        new_contract: ::core::clone::Clone::clone(__self_0),
                    }
                }
                Action::RegisterChain { chain: __self_0, emitter_address: __self_1 } => {
                    Action::RegisterChain {
                        chain: ::core::clone::Clone::clone(__self_0),
                        emitter_address: ::core::clone::Clone::clone(__self_1),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Action {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Action {
        #[inline]
        fn eq(&self, other: &Action) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        Action::ContractUpgrade { new_contract: __self_0 },
                        Action::ContractUpgrade { new_contract: __arg1_0 },
                    ) => *__self_0 == *__arg1_0,
                    (
                        Action::RegisterChain {
                            chain: __self_0,
                            emitter_address: __self_1,
                        },
                        Action::RegisterChain {
                            chain: __arg1_0,
                            emitter_address: __arg1_1,
                        },
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Action {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Action {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Address>;
            let _: ::core::cmp::AssertParamIsEq<Chain>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Action {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Action,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match (self, other) {
                        (
                            Action::ContractUpgrade { new_contract: __self_0 },
                            Action::ContractUpgrade { new_contract: __arg1_0 },
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            Action::RegisterChain {
                                chain: __self_0,
                                emitter_address: __self_1,
                            },
                            Action::RegisterChain {
                                chain: __arg1_0,
                                emitter_address: __arg1_1,
                            },
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                __self_0,
                                __arg1_0,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                                }
                                cmp => cmp,
                            }
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Action {
        #[inline]
        fn cmp(&self, other: &Action) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                ::core::cmp::Ordering::Equal => {
                    match (self, other) {
                        (
                            Action::ContractUpgrade { new_contract: __self_0 },
                            Action::ContractUpgrade { new_contract: __arg1_0 },
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            Action::RegisterChain {
                                chain: __self_0,
                                emitter_address: __self_1,
                            },
                            Action::RegisterChain {
                                chain: __arg1_0,
                                emitter_address: __arg1_1,
                            },
                        ) => {
                            match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                                }
                                cmp => cmp,
                            }
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Action {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Action::ContractUpgrade { new_contract: __self_0 } => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Action::RegisterChain { chain: __self_0, emitter_address: __self_1 } => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state)
                }
            }
        }
    }
    pub struct GovernancePacket {
        pub chain: Chain,
        pub action: Action,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GovernancePacket {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "GovernancePacket",
                "chain",
                &&self.chain,
                "action",
                &&self.action,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GovernancePacket {
        #[inline]
        fn clone(&self) -> GovernancePacket {
            GovernancePacket {
                chain: ::core::clone::Clone::clone(&self.chain),
                action: ::core::clone::Clone::clone(&self.action),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GovernancePacket {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GovernancePacket {
        #[inline]
        fn eq(&self, other: &GovernancePacket) -> bool {
            self.chain == other.chain && self.action == other.action
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for GovernancePacket {}
    #[automatically_derived]
    impl ::core::cmp::Eq for GovernancePacket {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Chain>;
            let _: ::core::cmp::AssertParamIsEq<Action>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for GovernancePacket {
        #[inline]
        fn partial_cmp(
            &self,
            other: &GovernancePacket,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.chain, &other.chain) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.action, &other.action)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for GovernancePacket {
        #[inline]
        fn cmp(&self, other: &GovernancePacket) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.chain, &other.chain) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.action, &other.action)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for GovernancePacket {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.chain, state);
            ::core::hash::Hash::hash(&self.action, state)
        }
    }
    mod governance_packet_impl {
        use std::fmt;
        use serde::{
            de::{Error, MapAccess, SeqAccess, Unexpected, Visitor},
            ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
        };
        use crate::{
            token::{Action, GovernancePacket},
            Address, Chain,
        };
        const MODULE: [u8; 32] = [
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x54,
            0x6f,
            0x6b,
            0x65,
            0x6e,
            0x42,
            0x72,
            0x69,
            0x64,
            0x67,
            0x65,
        ];
        struct Module;
        impl Serialize for Module {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                MODULE.serialize(serializer)
            }
        }
        impl<'de> Deserialize<'de> for Module {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let arr = <[u8; 32]>::deserialize(deserializer)?;
                if arr == MODULE {
                    Ok(Module)
                } else {
                    let expected = {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&MODULE)],
                            ),
                        );
                        res
                    };
                    Err(Error::invalid_value(Unexpected::Bytes(&arr), &&*expected))
                }
            }
        }
        struct ContractUpgrade {
            new_contract: Address,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ContractUpgrade {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ContractUpgrade",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "new_contract",
                        &self.new_contract,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ContractUpgrade {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "new_contract" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"new_contract" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ContractUpgrade>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ContractUpgrade;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ContractUpgrade",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Address,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ContractUpgrade with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ContractUpgrade {
                                new_contract: __field0,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Address> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "new_contract",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Address,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("new_contract") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ContractUpgrade {
                                new_contract: __field0,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["new_contract"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ContractUpgrade",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ContractUpgrade>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        struct RegisterChain {
            chain: Chain,
            emitter_address: Address,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for RegisterChain {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "RegisterChain",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "chain",
                        &self.chain,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "emitter_address",
                        &self.emitter_address,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for RegisterChain {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "chain" => _serde::__private::Ok(__Field::__field0),
                                "emitter_address" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"chain" => _serde::__private::Ok(__Field::__field0),
                                b"emitter_address" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<RegisterChain>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = RegisterChain;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct RegisterChain",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Chain,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct RegisterChain with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Address,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct RegisterChain with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(RegisterChain {
                                chain: __field0,
                                emitter_address: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Chain> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Address> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("chain"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Chain,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "emitter_address",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Address,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("chain") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "emitter_address",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(RegisterChain {
                                chain: __field0,
                                emitter_address: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "chain",
                        "emitter_address",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "RegisterChain",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<RegisterChain>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Serialize for GovernancePacket {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut seq = serializer.serialize_struct("GovernancePacket", 4)?;
                seq.serialize_field("module", &Module)?;
                match self.action {
                    Action::ContractUpgrade { new_contract } => {
                        seq.serialize_field("action", &1u8)?;
                        seq.serialize_field("chain", &self.chain)?;
                        seq.serialize_field(
                            "payload",
                            &ContractUpgrade { new_contract },
                        )?;
                    }
                    Action::RegisterChain { chain, emitter_address } => {
                        seq.serialize_field("action", &2u8)?;
                        seq.serialize_field("chain", &self.chain)?;
                        seq.serialize_field(
                            "payload",
                            &RegisterChain {
                                chain,
                                emitter_address,
                            },
                        )?;
                    }
                }
                seq.end()
            }
        }
        struct GovernancePacketVisitor;
        impl<'de> Visitor<'de> for GovernancePacketVisitor {
            type Value = GovernancePacket;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("struct GovernancePacket")
            }
            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                static EXPECTING: &str = "struct GovernancePacket with 4 elements";
                let _: Module = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &EXPECTING))?;
                let act: u8 = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &EXPECTING))?;
                let chain = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(2, &EXPECTING))?;
                let action = match act {
                    1 => {
                        let ContractUpgrade { new_contract } = seq
                            .next_element()?
                            .ok_or_else(|| Error::invalid_length(3, &EXPECTING))?;
                        Action::ContractUpgrade {
                            new_contract,
                        }
                    }
                    2 => {
                        let RegisterChain { chain, emitter_address } = seq
                            .next_element()?
                            .ok_or_else(|| Error::invalid_length(3, &EXPECTING))?;
                        Action::RegisterChain {
                            chain,
                            emitter_address,
                        }
                    }
                    v => {
                        return Err(
                            Error::invalid_value(
                                Unexpected::Unsigned(v.into()),
                                &"one of 1, 2",
                            ),
                        );
                    }
                };
                Ok(GovernancePacket { chain, action })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                #[serde(rename_all = "snake_case")]
                enum Field {
                    Module,
                    Action,
                    Chain,
                    Payload,
                }
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl _serde::Serialize for Field {
                        fn serialize<__S>(
                            &self,
                            __serializer: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            match *self {
                                Field::Module => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "Field",
                                        0u32,
                                        "module",
                                    )
                                }
                                Field::Action => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "Field",
                                        1u32,
                                        "action",
                                    )
                                }
                                Field::Chain => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "Field",
                                        2u32,
                                        "chain",
                                    )
                                }
                                Field::Payload => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "Field",
                                        3u32,
                                        "payload",
                                    )
                                }
                            }
                        }
                    }
                };
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for Field {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "variant identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                        2u64 => _serde::__private::Ok(__Field::__field2),
                                        3u64 => _serde::__private::Ok(__Field::__field3),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"variant index 0 <= i < 4",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "module" => _serde::__private::Ok(__Field::__field0),
                                        "action" => _serde::__private::Ok(__Field::__field1),
                                        "chain" => _serde::__private::Ok(__Field::__field2),
                                        "payload" => _serde::__private::Ok(__Field::__field3),
                                        _ => {
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"module" => _serde::__private::Ok(__Field::__field0),
                                        b"action" => _serde::__private::Ok(__Field::__field1),
                                        b"chain" => _serde::__private::Ok(__Field::__field2),
                                        b"payload" => _serde::__private::Ok(__Field::__field3),
                                        _ => {
                                            let __value = &_serde::__private::from_utf8_lossy(__value);
                                            _serde::__private::Err(
                                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                                            )
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<Field>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "enum Field",
                                    )
                                }
                                fn visit_enum<__A>(
                                    self,
                                    __data: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::EnumAccess<'de>,
                                {
                                    match match _serde::de::EnumAccess::variant(__data) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        (__Field::__field0, __variant) => {
                                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                            _serde::__private::Ok(Field::Module)
                                        }
                                        (__Field::__field1, __variant) => {
                                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                            _serde::__private::Ok(Field::Action)
                                        }
                                        (__Field::__field2, __variant) => {
                                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                            _serde::__private::Ok(Field::Chain)
                                        }
                                        (__Field::__field3, __variant) => {
                                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                            _serde::__private::Ok(Field::Payload)
                                        }
                                    }
                                }
                            }
                            const VARIANTS: &'static [&'static str] = &[
                                "module",
                                "action",
                                "chain",
                                "payload",
                            ];
                            _serde::Deserializer::deserialize_enum(
                                __deserializer,
                                "Field",
                                VARIANTS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<Field>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                };
                let mut module = None;
                let mut chain = None;
                let mut action = None;
                let mut payload = None;
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Module => {
                            if module.is_some() {
                                return Err(Error::duplicate_field("module"));
                            }
                            module = map.next_value::<Module>().map(Some)?;
                        }
                        Field::Action => {
                            if action.is_some() {
                                return Err(Error::duplicate_field("action"));
                            }
                            action = map.next_value::<u8>().map(Some)?;
                        }
                        Field::Chain => {
                            if chain.is_some() {
                                return Err(Error::duplicate_field("chain"));
                            }
                            chain = map.next_value().map(Some)?;
                        }
                        Field::Payload => {
                            if payload.is_some() {
                                return Err(Error::duplicate_field("payload"));
                            }
                            let a = action
                                .as_ref()
                                .copied()
                                .ok_or_else(|| {
                                    Error::custom(
                                        "`action` must be known before deserializing `payload`",
                                    )
                                })?;
                            let p = match a {
                                1 => {
                                    let ContractUpgrade { new_contract } = map.next_value()?;
                                    Action::ContractUpgrade {
                                        new_contract,
                                    }
                                }
                                2 => {
                                    let RegisterChain { chain, emitter_address } = map
                                        .next_value()?;
                                    Action::RegisterChain {
                                        chain,
                                        emitter_address,
                                    }
                                }
                                v => {
                                    return Err(
                                        Error::custom(
                                            ::core::fmt::Arguments::new_v1(
                                                &["invalid action: ", ", expected one of: 1, 2"],
                                                &[::core::fmt::ArgumentV1::new_display(&v)],
                                            ),
                                        ),
                                    );
                                }
                            };
                            payload = Some(p);
                        }
                    }
                }
                let _ = module.ok_or_else(|| Error::missing_field("module"))?;
                let chain = chain.ok_or_else(|| Error::missing_field("chain"))?;
                let action = payload.ok_or_else(|| Error::missing_field("payload"))?;
                Ok(GovernancePacket { chain, action })
            }
        }
        impl<'de> Deserialize<'de> for GovernancePacket {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                const FIELDS: &'static [&'static str] = &[
                    "module",
                    "action",
                    "chain",
                    "payload",
                ];
                deserializer
                    .deserialize_struct(
                        "GovernancePacket",
                        FIELDS,
                        GovernancePacketVisitor,
                    )
            }
        }
    }
}
pub mod vaa {
    //! VAA's represent a collection of signatures combined with a message and its metadata. VAA's are
    //! used as a form of proof; by submitting a VAA to a target contract, the receiving contract can
    //! make assumptions about the validity of state on the source chain.
    //!
    //! Wormhole defines several VAA's for use within Token/NFT bridge implemenetations, as well as
    //! governance specific VAA's used within Wormhole's guardian network.
    //!
    //! This module provides definitions and parsers for all current Wormhole standard VAA's, and
    //! includes parsers for the core VAA type. Programs targetting wormhole can use this module to
    //! parse and verify incoming VAA's securely.
    use serde::{Deserialize, Serialize};
    use crate::{Address, Chain};
    /// Signatures are typical ECDSA signatures prefixed with a Guardian position.
    pub struct Signature {
        pub index: u8,
        #[serde(with = "crate::serde_array")]
        pub signature: [u8; 65],
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Signature {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Signature",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "index",
                    &self.index,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "signature",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a [u8; 65],),
                            phantom: _serde::__private::PhantomData<Signature>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                crate::serde_array::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.signature,),
                            phantom: _serde::__private::PhantomData::<Signature>,
                        }
                    },
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Signature {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "index" => _serde::__private::Ok(__Field::__field0),
                            "signature" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"index" => _serde::__private::Ok(__Field::__field0),
                            b"signature" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Signature>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Signature;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Signature",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            u8,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Signature with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match {
                            struct __DeserializeWith<'de> {
                                value: [u8; 65],
                                phantom: _serde::__private::PhantomData<Signature>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de>
                            for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: match crate::serde_array::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<
                                    __DeserializeWith<'de>,
                                >(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Signature with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Signature {
                            index: __field0,
                            signature: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<[u8; 65]> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "signature",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some({
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 65],
                                            phantom: _serde::__private::PhantomData<Signature>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: match crate::serde_array::deserialize(
                                                        __deserializer,
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    },
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("index") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "signature",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Signature {
                            index: __field0,
                            signature: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["index", "signature"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Signature",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Signature>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Signature {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Signature",
                "index",
                &&self.index,
                "signature",
                &&self.signature,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Signature {
        #[inline]
        fn clone(&self) -> Signature {
            let _: ::core::clone::AssertParamIsClone<u8>;
            let _: ::core::clone::AssertParamIsClone<[u8; 65]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Signature {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Signature {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Signature {
        #[inline]
        fn eq(&self, other: &Signature) -> bool {
            self.index == other.index && self.signature == other.signature
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Signature {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Signature {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<[u8; 65]>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Signature {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Signature,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.index, &other.index) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(
                        &self.signature,
                        &other.signature,
                    )
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Signature {
        #[inline]
        fn cmp(&self, other: &Signature) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.index, &other.index) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.signature, &other.signature)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Signature {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.index, state);
            ::core::hash::Hash::hash(&self.signature, state)
        }
    }
    impl Default for Signature {
        fn default() -> Self {
            Self {
                index: 0,
                signature: [0; 65],
            }
        }
    }
    /// The core VAA itself. This structure is what is received by a contract on the receiving side of
    /// a wormhole message passing flow.
    pub struct Vaa<P> {
        #[serde(flatten)]
        pub header: Header,
        #[serde(flatten)]
        pub body: Body<P>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<P> _serde::Serialize for Vaa<P>
        where
            P: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_map(
                    __serializer,
                    _serde::__private::None,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::Serialize::serialize(
                    &&self.header,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::Serialize::serialize(
                    &&self.body,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, P> _serde::Deserialize<'de> for Vaa<P>
        where
            P: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ),
                        )
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::I8(__value)),
                        )
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ),
                        )
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ),
                        )
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ),
                        )
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::U8(__value)),
                        )
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ),
                        )
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ),
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ),
                        )
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ),
                        )
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ),
                        )
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ),
                        )
                    }
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::Unit),
                        )
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::__private::de::Content::ByteBuf(
                                    __value.to_vec(),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(
                                    __value,
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de, P>
                where
                    P: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<Vaa<P>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, P> _serde::de::Visitor<'de> for __Visitor<'de, P>
                where
                    P: _serde::Deserialize<'de>,
                {
                    type Value = Vaa<P>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Vaa",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<
                                (
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                ),
                            >,
                        >::new();
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__other(__name) => {
                                    __collect
                                        .push(
                                            _serde::__private::Some((
                                                __name,
                                                match _serde::de::MapAccess::next_value(&mut __map) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            )),
                                        );
                                }
                            }
                        }
                        let __field0: Header = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        let __field1: Body<P> = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Vaa {
                            header: __field0,
                            body: __field1,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Vaa<P>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl<P: ::core::fmt::Debug> ::core::fmt::Debug for Vaa<P> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Vaa",
                "header",
                &&self.header,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl<P: ::core::default::Default> ::core::default::Default for Vaa<P> {
        #[inline]
        fn default() -> Vaa<P> {
            Vaa {
                header: ::core::default::Default::default(),
                body: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl<P: ::core::clone::Clone> ::core::clone::Clone for Vaa<P> {
        #[inline]
        fn clone(&self) -> Vaa<P> {
            Vaa {
                header: ::core::clone::Clone::clone(&self.header),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    #[automatically_derived]
    impl<P> ::core::marker::StructuralPartialEq for Vaa<P> {}
    #[automatically_derived]
    impl<P: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Vaa<P> {
        #[inline]
        fn eq(&self, other: &Vaa<P>) -> bool {
            self.header == other.header && self.body == other.body
        }
    }
    #[automatically_derived]
    impl<P> ::core::marker::StructuralEq for Vaa<P> {}
    #[automatically_derived]
    impl<P: ::core::cmp::Eq> ::core::cmp::Eq for Vaa<P> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Header>;
            let _: ::core::cmp::AssertParamIsEq<Body<P>>;
        }
    }
    #[automatically_derived]
    impl<P: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for Vaa<P> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Vaa<P>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.header, &other.header) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.body, &other.body)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl<P: ::core::cmp::Ord> ::core::cmp::Ord for Vaa<P> {
        #[inline]
        fn cmp(&self, other: &Vaa<P>) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.header, &other.header) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.body, &other.body)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl<P: ::core::hash::Hash> ::core::hash::Hash for Vaa<P> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.header, state);
            ::core::hash::Hash::hash(&self.body, state)
        }
    }
    /// The header for a VAA.
    pub struct Header {
        pub version: u8,
        pub guardian_set_index: u32,
        pub signatures: Vec<Signature>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Header {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Header",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "guardian_set_index",
                    &self.guardian_set_index,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "signatures",
                    &self.signatures,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Header {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "version" => _serde::__private::Ok(__Field::__field0),
                            "guardian_set_index" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            "signatures" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"version" => _serde::__private::Ok(__Field::__field0),
                            b"guardian_set_index" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            b"signatures" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Header>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Header;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Header",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            u8,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Header with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Header with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Signature>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Header with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Header {
                            version: __field0,
                            guardian_set_index: __field1,
                            signatures: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<Signature>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "guardian_set_index",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "signatures",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<Signature>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("version") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "guardian_set_index",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("signatures") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Header {
                            version: __field0,
                            guardian_set_index: __field1,
                            signatures: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "version",
                    "guardian_set_index",
                    "signatures",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Header",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Header>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Header {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Header",
                "version",
                &&self.version,
                "guardian_set_index",
                &&self.guardian_set_index,
                "signatures",
                &&self.signatures,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Header {
        #[inline]
        fn default() -> Header {
            Header {
                version: ::core::default::Default::default(),
                guardian_set_index: ::core::default::Default::default(),
                signatures: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Header {
        #[inline]
        fn clone(&self) -> Header {
            Header {
                version: ::core::clone::Clone::clone(&self.version),
                guardian_set_index: ::core::clone::Clone::clone(
                    &self.guardian_set_index,
                ),
                signatures: ::core::clone::Clone::clone(&self.signatures),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Header {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Header {
        #[inline]
        fn eq(&self, other: &Header) -> bool {
            self.version == other.version
                && self.guardian_set_index == other.guardian_set_index
                && self.signatures == other.signatures
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Header {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Header {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<Vec<Signature>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Header {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Header,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.version, &other.version) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.guardian_set_index,
                        &other.guardian_set_index,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(
                                &self.signatures,
                                &other.signatures,
                            )
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Header {
        #[inline]
        fn cmp(&self, other: &Header) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.version, &other.version) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(
                        &self.guardian_set_index,
                        &other.guardian_set_index,
                    ) {
                        ::core::cmp::Ordering::Equal => {
                            ::core::cmp::Ord::cmp(&self.signatures, &other.signatures)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Header {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.version, state);
            ::core::hash::Hash::hash(&self.guardian_set_index, state);
            ::core::hash::Hash::hash(&self.signatures, state)
        }
    }
    pub struct Body<P> {
        /// Seconds since UNIX epoch.
        pub timestamp: u32,
        pub nonce: u32,
        pub emitter_chain: Chain,
        pub emitter_address: Address,
        pub sequence: u64,
        pub consistency_level: u8,
        pub payload: P,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<P> _serde::Serialize for Body<P>
        where
            P: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Body",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "timestamp",
                    &self.timestamp,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "nonce",
                    &self.nonce,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "emitter_chain",
                    &self.emitter_chain,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "emitter_address",
                    &self.emitter_address,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "sequence",
                    &self.sequence,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "consistency_level",
                    &self.consistency_level,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "payload",
                    &self.payload,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, P> _serde::Deserialize<'de> for Body<P>
        where
            P: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "timestamp" => _serde::__private::Ok(__Field::__field0),
                            "nonce" => _serde::__private::Ok(__Field::__field1),
                            "emitter_chain" => _serde::__private::Ok(__Field::__field2),
                            "emitter_address" => _serde::__private::Ok(__Field::__field3),
                            "sequence" => _serde::__private::Ok(__Field::__field4),
                            "consistency_level" => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            "payload" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"timestamp" => _serde::__private::Ok(__Field::__field0),
                            b"nonce" => _serde::__private::Ok(__Field::__field1),
                            b"emitter_chain" => _serde::__private::Ok(__Field::__field2),
                            b"emitter_address" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"sequence" => _serde::__private::Ok(__Field::__field4),
                            b"consistency_level" => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            b"payload" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de, P>
                where
                    P: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<Body<P>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, P> _serde::de::Visitor<'de> for __Visitor<'de, P>
                where
                    P: _serde::Deserialize<'de>,
                {
                    type Value = Body<P>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Body",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Body with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Body with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Chain,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Body with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Address,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Body with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            u64,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Body with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            u8,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Body with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            P,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Body with 7 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Body {
                            timestamp: __field0,
                            nonce: __field1,
                            emitter_chain: __field2,
                            emitter_address: __field3,
                            sequence: __field4,
                            consistency_level: __field5,
                            payload: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Chain> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Address> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<P> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timestamp",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("nonce"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "emitter_chain",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Chain,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "emitter_address",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Address,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "sequence",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "consistency_level",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<P>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("timestamp") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("nonce") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "emitter_chain",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "emitter_address",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("sequence") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "consistency_level",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("payload") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Body {
                            timestamp: __field0,
                            nonce: __field1,
                            emitter_chain: __field2,
                            emitter_address: __field3,
                            sequence: __field4,
                            consistency_level: __field5,
                            payload: __field6,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "timestamp",
                    "nonce",
                    "emitter_chain",
                    "emitter_address",
                    "sequence",
                    "consistency_level",
                    "payload",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Body",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Body<P>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl<P: ::core::fmt::Debug> ::core::fmt::Debug for Body<P> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "timestamp",
                "nonce",
                "emitter_chain",
                "emitter_address",
                "sequence",
                "consistency_level",
                "payload",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.timestamp,
                &&self.nonce,
                &&self.emitter_chain,
                &&self.emitter_address,
                &&self.sequence,
                &&self.consistency_level,
                &&self.payload,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Body", names, values)
        }
    }
    #[automatically_derived]
    impl<P: ::core::default::Default> ::core::default::Default for Body<P> {
        #[inline]
        fn default() -> Body<P> {
            Body {
                timestamp: ::core::default::Default::default(),
                nonce: ::core::default::Default::default(),
                emitter_chain: ::core::default::Default::default(),
                emitter_address: ::core::default::Default::default(),
                sequence: ::core::default::Default::default(),
                consistency_level: ::core::default::Default::default(),
                payload: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl<P: ::core::clone::Clone> ::core::clone::Clone for Body<P> {
        #[inline]
        fn clone(&self) -> Body<P> {
            Body {
                timestamp: ::core::clone::Clone::clone(&self.timestamp),
                nonce: ::core::clone::Clone::clone(&self.nonce),
                emitter_chain: ::core::clone::Clone::clone(&self.emitter_chain),
                emitter_address: ::core::clone::Clone::clone(&self.emitter_address),
                sequence: ::core::clone::Clone::clone(&self.sequence),
                consistency_level: ::core::clone::Clone::clone(&self.consistency_level),
                payload: ::core::clone::Clone::clone(&self.payload),
            }
        }
    }
    #[automatically_derived]
    impl<P> ::core::marker::StructuralPartialEq for Body<P> {}
    #[automatically_derived]
    impl<P: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Body<P> {
        #[inline]
        fn eq(&self, other: &Body<P>) -> bool {
            self.timestamp == other.timestamp && self.nonce == other.nonce
                && self.emitter_chain == other.emitter_chain
                && self.emitter_address == other.emitter_address
                && self.sequence == other.sequence
                && self.consistency_level == other.consistency_level
                && self.payload == other.payload
        }
    }
    #[automatically_derived]
    impl<P> ::core::marker::StructuralEq for Body<P> {}
    #[automatically_derived]
    impl<P: ::core::cmp::Eq> ::core::cmp::Eq for Body<P> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<Chain>;
            let _: ::core::cmp::AssertParamIsEq<Address>;
            let _: ::core::cmp::AssertParamIsEq<u64>;
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<P>;
        }
    }
    #[automatically_derived]
    impl<P: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for Body<P> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Body<P>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(
                &self.timestamp,
                &other.timestamp,
            ) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.nonce,
                        &other.nonce,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.emitter_chain,
                                &other.emitter_chain,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        &self.emitter_address,
                                        &other.emitter_address,
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => {
                                            match ::core::cmp::PartialOrd::partial_cmp(
                                                &self.sequence,
                                                &other.sequence,
                                            ) {
                                                ::core::option::Option::Some(
                                                    ::core::cmp::Ordering::Equal,
                                                ) => {
                                                    match ::core::cmp::PartialOrd::partial_cmp(
                                                        &self.consistency_level,
                                                        &other.consistency_level,
                                                    ) {
                                                        ::core::option::Option::Some(
                                                            ::core::cmp::Ordering::Equal,
                                                        ) => {
                                                            ::core::cmp::PartialOrd::partial_cmp(
                                                                &self.payload,
                                                                &other.payload,
                                                            )
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl<P: ::core::cmp::Ord> ::core::cmp::Ord for Body<P> {
        #[inline]
        fn cmp(&self, other: &Body<P>) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.timestamp, &other.timestamp) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(&self.nonce, &other.nonce) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(
                                &self.emitter_chain,
                                &other.emitter_chain,
                            ) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(
                                        &self.emitter_address,
                                        &other.emitter_address,
                                    ) {
                                        ::core::cmp::Ordering::Equal => {
                                            match ::core::cmp::Ord::cmp(
                                                &self.sequence,
                                                &other.sequence,
                                            ) {
                                                ::core::cmp::Ordering::Equal => {
                                                    match ::core::cmp::Ord::cmp(
                                                        &self.consistency_level,
                                                        &other.consistency_level,
                                                    ) {
                                                        ::core::cmp::Ordering::Equal => {
                                                            ::core::cmp::Ord::cmp(&self.payload, &other.payload)
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl<P: ::core::hash::Hash> ::core::hash::Hash for Body<P> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.timestamp, state);
            ::core::hash::Hash::hash(&self.nonce, state);
            ::core::hash::Hash::hash(&self.emitter_chain, state);
            ::core::hash::Hash::hash(&self.emitter_address, state);
            ::core::hash::Hash::hash(&self.sequence, state);
            ::core::hash::Hash::hash(&self.consistency_level, state);
            ::core::hash::Hash::hash(&self.payload, state)
        }
    }
    /// Digest data for the VAA.
    pub struct Digest {
        /// Guardians don't hash the VAA body directly, instead they hash the VAA and sign the hash. The
        /// purpose of this is it means when submitting a VAA on-chain we only have to submit the hash
        /// which reduces gas costs.
        pub hash: [u8; 32],
        /// The secp256k_hash is the hash of the hash of the VAA. The reason we provide this is because
        /// of how secp256k works internally. It hashes its payload before signing. This means that
        /// when verifying secp256k signatures, we're actually checking if a guardian has signed the
        /// hash of the hash of the VAA. Functions such as `ecrecover` expect the secp256k hash rather
        /// than the original payload.
        pub secp256k_hash: [u8; 32],
    }
    impl<P> Vaa<P> {
        /// Check if the VAA is a Governance VAA.
        pub fn is_governance(&self) -> bool {
            self.body.emitter_address == crate::GOVERNANCE_EMITTER
                && self.body.emitter_chain == Chain::Solana
        }
    }
    impl<P: Serialize> Vaa<P> {
        /// VAA Digest Components.
        ///
        /// A VAA is distinguished by the unique 256bit Keccak256 hash of its body. This hash is
        /// utilised in all Wormhole components for identifying unique VAA's, including the bridge,
        /// modules, and core guardian software. The `Digest` is documented with reasoning for
        /// each field.
        ///
        /// NOTE: This function uses a library to do Keccak256 hashing, but on-chain this may not be
        /// efficient. If efficiency is needed, consider calling `serde_wormhole::to_writer` instead
        /// and hashing the result using on-chain primitives.
        #[inline]
        pub fn digest(&self) -> anyhow::Result<Digest> {
            self.body.digest()
        }
        /// Like `digest` but allows specifying an additional payload to include in the body hash.
        #[inline]
        pub fn digest_with_payload(&self, payload: &[u8]) -> anyhow::Result<Digest> {
            self.body.digest_with_payload(payload)
        }
    }
    impl Header {
        pub fn verify(&self, _body: &[u8]) -> anyhow::Result<()> {
            Ok(())
        }
    }
    impl<P: Serialize> Body<P> {
        /// Body Digest Components.
        ///
        /// A VAA is distinguished by the unique 256bit Keccak256 hash of its body. This hash is
        /// utilised in all Wormhole components for identifying unique VAA's, including the bridge,
        /// modules, and core guardian software. The `Digest` is documented with reasoning for
        /// each field.
        ///
        /// NOTE: This function uses a library to do Keccak256 hashing, but on-chain this may not be
        /// efficient. If efficiency is needed, consider calling `serde_wormhole::to_writer` instead
        /// and hashing the result using on-chain primitives.
        #[inline]
        pub fn digest(&self) -> anyhow::Result<Digest> {
            self.digest_with_payload(&[])
        }
        /// Like `digest` but allows specifying an additional payload to include in the body hash.
        pub fn digest_with_payload(&self, payload: &[u8]) -> anyhow::Result<Digest> {
            use std::io::Write;
            use anyhow::Context;
            use sha3::Digest as Sha3Digest;
            let hash: [u8; 32] = {
                let mut h = sha3::Keccak256::default();
                serde_wormhole::to_writer(&mut h, self)
                    .context("failed to serialize body")?;
                h.write_all(payload).context("failed to hash extra payload")?;
                h.finalize().into()
            };
            let secp256k_hash: [u8; 32] = {
                let mut h = sha3::Keccak256::default();
                let _ = h.write_all(&hash).unwrap();
                h.finalize().into()
            };
            Ok(Digest { hash, secp256k_hash })
        }
    }
}
pub use {chain::Chain, vaa::Vaa};
/// The `GOVERNANCE_EMITTER` is a special address Wormhole guardians trust to observe governance
/// actions from. The value is "0000000000000000000000000000000000000000000000000000000000000004".
pub const GOVERNANCE_EMITTER: Address = Address([
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x04,
]);
pub struct GuardianAddress(pub [u8; 20]);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for GuardianAddress {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(
                __serializer,
                "GuardianAddress",
                &self.0,
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for GuardianAddress {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<GuardianAddress>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GuardianAddress;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct GuardianAddress",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: [u8; 20] = match <[u8; 20] as _serde::Deserialize>::deserialize(
                        __e,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(GuardianAddress(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        [u8; 20],
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct GuardianAddress with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(GuardianAddress(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "GuardianAddress",
                __Visitor {
                    marker: _serde::__private::PhantomData::<GuardianAddress>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for GuardianAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "GuardianAddress", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for GuardianAddress {
    #[inline]
    fn clone(&self) -> GuardianAddress {
        let _: ::core::clone::AssertParamIsClone<[u8; 20]>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for GuardianAddress {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for GuardianAddress {}
#[automatically_derived]
impl ::core::cmp::PartialEq for GuardianAddress {
    #[inline]
    fn eq(&self, other: &GuardianAddress) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for GuardianAddress {}
#[automatically_derived]
impl ::core::cmp::Eq for GuardianAddress {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 20]>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for GuardianAddress {
    #[inline]
    fn partial_cmp(
        &self,
        other: &GuardianAddress,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for GuardianAddress {
    #[inline]
    fn cmp(&self, other: &GuardianAddress) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for GuardianAddress {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
/// Wormhole specifies addresses as 32 bytes. Addresses that are shorter, for example 20 byte
/// Ethereum addresses, are left zero padded to 32.
pub struct Address(pub [u8; 32]);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Address {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(
                __serializer,
                "Address",
                &self.0,
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Address {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Address>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Address;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct Address",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: [u8; 32] = match <[u8; 32] as _serde::Deserialize>::deserialize(
                        __e,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(Address(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        [u8; 32],
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct Address with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Address(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "Address",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Address>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for Address {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Address", &&self.0)
    }
}
#[automatically_derived]
impl ::core::default::Default for Address {
    #[inline]
    fn default() -> Address {
        Address(::core::default::Default::default())
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Address {
    #[inline]
    fn clone(&self) -> Address {
        let _: ::core::clone::AssertParamIsClone<[u8; 32]>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Address {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Address {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Address {
    #[inline]
    fn eq(&self, other: &Address) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Address {}
#[automatically_derived]
impl ::core::cmp::Eq for Address {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Address {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Address,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Address {
    #[inline]
    fn cmp(&self, other: &Address) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Address {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
/// Wormhole specifies an amount as a uint256 encoded in big-endian order.
pub struct Amount(pub [u8; 32]);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Amount {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "Amount", &self.0)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Amount {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Amount>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Amount;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct Amount",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: [u8; 32] = match <[u8; 32] as _serde::Deserialize>::deserialize(
                        __e,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(Amount(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        [u8; 32],
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct Amount with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Amount(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "Amount",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Amount>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for Amount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Amount", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Amount {
    #[inline]
    fn clone(&self) -> Amount {
        let _: ::core::clone::AssertParamIsClone<[u8; 32]>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Amount {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Amount {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Amount {
    #[inline]
    fn eq(&self, other: &Amount) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Amount {}
#[automatically_derived]
impl ::core::cmp::Eq for Amount {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Amount {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Amount,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Amount {
    #[inline]
    fn cmp(&self, other: &Amount) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Amount {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
/// A `GuardianSet` is a versioned set of keys that can sign Wormhole messages.
pub struct GuardianSetInfo {
    /// The set of guardians public keys, in Ethereum's compressed format.
    pub addresses: Vec<GuardianAddress>,
    /// How long after a GuardianSet change before this set is expired.
    #[serde(skip)]
    pub expiration_time: u64,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for GuardianSetInfo {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GuardianSetInfo",
                false as usize + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "addresses",
                &self.addresses,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for GuardianSetInfo {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "addresses" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"addresses" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<GuardianSetInfo>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GuardianSetInfo;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct GuardianSetInfo",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        Vec<GuardianAddress>,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GuardianSetInfo with 1 element",
                                ),
                            );
                        }
                    };
                    let __field1 = _serde::__private::Default::default();
                    _serde::__private::Ok(GuardianSetInfo {
                        addresses: __field0,
                        expiration_time: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<Vec<GuardianAddress>> = _serde::__private::None;
                    while let _serde::__private::Some(__key)
                        = match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "addresses",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        Vec<GuardianAddress>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("addresses") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(GuardianSetInfo {
                        addresses: __field0,
                        expiration_time: _serde::__private::Default::default(),
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["addresses"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GuardianSetInfo",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<GuardianSetInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for GuardianSetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "GuardianSetInfo",
            "addresses",
            &&self.addresses,
            "expiration_time",
            &&self.expiration_time,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for GuardianSetInfo {
    #[inline]
    fn clone(&self) -> GuardianSetInfo {
        GuardianSetInfo {
            addresses: ::core::clone::Clone::clone(&self.addresses),
            expiration_time: ::core::clone::Clone::clone(&self.expiration_time),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for GuardianSetInfo {}
#[automatically_derived]
impl ::core::cmp::PartialEq for GuardianSetInfo {
    #[inline]
    fn eq(&self, other: &GuardianSetInfo) -> bool {
        self.addresses == other.addresses
            && self.expiration_time == other.expiration_time
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for GuardianSetInfo {}
#[automatically_derived]
impl ::core::cmp::Eq for GuardianSetInfo {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Vec<GuardianAddress>>;
        let _: ::core::cmp::AssertParamIsEq<u64>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for GuardianSetInfo {
    #[inline]
    fn partial_cmp(
        &self,
        other: &GuardianSetInfo,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.addresses, &other.addresses) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                ::core::cmp::PartialOrd::partial_cmp(
                    &self.expiration_time,
                    &other.expiration_time,
                )
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for GuardianSetInfo {
    #[inline]
    fn cmp(&self, other: &GuardianSetInfo) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.addresses, &other.addresses) {
            ::core::cmp::Ordering::Equal => {
                ::core::cmp::Ord::cmp(&self.expiration_time, &other.expiration_time)
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for GuardianSetInfo {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.addresses, state);
        ::core::hash::Hash::hash(&self.expiration_time, state)
    }
}
impl GuardianSetInfo {
    pub fn quorum(&self) -> usize {
        if self.addresses.is_empty() {
            0
        } else {
            ((self.addresses.len() * 10 / 3) * 2) / 10 + 1
        }
    }
}
