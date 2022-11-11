//! Parsers for Token bridge VAAs.
//!
//! Token bridging relies on VAA's that indicate custody/lockup/burn events in order to maintain
//! token parity between multiple chains. These parsers can be used to read these VAAs. It also
//! defines the Governance actions that this module supports, namely contract upgrades and chain
//! registrations.

use bstr::BString;
use serde::{Deserialize, Serialize};

use crate::{Address, Amount, Chain};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        // The payload is directly appended to the message.
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    RegisterChain {
        chain: Chain,
        emitter_address: Address,
    },
    ContractUpgrade {
        new_contract: Address,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GovernancePacket {
    pub chain: Chain,
    pub action: Action,
}

// The wire format for GovernancePackets is wonky and doesn't lend itself well to auto-deriving
// Serialize / Deserialize so we implement it manually here.
mod governance_packet_impl {
    use std::fmt;

    use serde::{
        de::{Error, MapAccess, SeqAccess, Unexpected, Visitor},
        ser::SerializeStruct,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use crate::{
        token::{Action, GovernancePacket},
        Address, Chain,
    };

    // MODULE = "TokenBridge"
    const MODULE: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x42, 0x72, 0x69, 0x64,
        0x67, 0x65,
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
                let expected = format!("{MODULE:?}");
                Err(Error::invalid_value(Unexpected::Bytes(&arr), &&*expected))
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    struct ContractUpgrade {
        new_contract: Address,
    }

    #[derive(Serialize, Deserialize)]
    struct RegisterChain {
        chain: Chain,
        emitter_address: Address,
    }

    impl Serialize for GovernancePacket {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_struct("GovernancePacket", 4)?;
            seq.serialize_field("module", &Module)?;

            // The wire format encodes the action before the chain and then appends the actual
            // action payload.
            match self.action {
                Action::RegisterChain {
                    chain,
                    emitter_address,
                } => {
                    seq.serialize_field("action", &1u8)?;
                    seq.serialize_field("chain", &self.chain)?;
                    seq.serialize_field(
                        "payload",
                        &RegisterChain {
                            chain,
                            emitter_address,
                        },
                    )?;
                }
                Action::ContractUpgrade { new_contract } => {
                    seq.serialize_field("action", &2u8)?;
                    seq.serialize_field("chain", &self.chain)?;
                    seq.serialize_field("payload", &ContractUpgrade { new_contract })?;
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
                    let RegisterChain {
                        chain,
                        emitter_address,
                    } = seq
                        .next_element()?
                        .ok_or_else(|| Error::invalid_length(3, &EXPECTING))?;

                    Action::RegisterChain {
                        chain,
                        emitter_address,
                    }
                }
                2 => {
                    let ContractUpgrade { new_contract } = seq
                        .next_element()?
                        .ok_or_else(|| Error::invalid_length(3, &EXPECTING))?;

                    Action::ContractUpgrade { new_contract }
                }
                v => {
                    return Err(Error::invalid_value(
                        Unexpected::Unsigned(v.into()),
                        &"one of 1, 2",
                    ))
                }
            };

            Ok(GovernancePacket { chain, action })
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "snake_case")]
            enum Field {
                Module,
                Action,
                Chain,
                Payload,
            }

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

                        let a = action.as_ref().copied().ok_or_else(|| {
                            Error::custom("`action` must be known before deserializing `payload`")
                        })?;

                        let p = match a {
                            1 => {
                                let ContractUpgrade { new_contract } = map.next_value()?;

                                Action::ContractUpgrade { new_contract }
                            }
                            2 => {
                                let RegisterChain {
                                    chain,
                                    emitter_address,
                                } = map.next_value()?;

                                Action::RegisterChain {
                                    chain,
                                    emitter_address,
                                }
                            }
                            v => {
                                return Err(Error::custom(format_args!(
                                    "invalid action: {v}, expected one of: 1, 2"
                                )))
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
            const FIELDS: &'static [&'static str] = &["module", "action", "chain", "payload"];
            deserializer.deserialize_struct("GovernancePacket", FIELDS, GovernancePacketVisitor)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{vaa::Signature, Vaa, GOVERNANCE_EMITTER};

    use super::*;

    #[test]
    pub fn transfer() {
        let payload = [
            0x01, 0xfa, 0x22, 0x8b, 0x3d, 0xf3, 0x59, 0x21, 0xa1, 0xc9, 0x39, 0xad, 0x9c, 0x54,
            0xe1, 0x2f, 0x87, 0xc6, 0x27, 0x25, 0xd1, 0xaf, 0x7c, 0x7b, 0x6a, 0xea, 0xbf, 0x48,
            0x14, 0xe7, 0x26, 0x56, 0xfa, 0xe8, 0xf2, 0xd2, 0x53, 0xd1, 0x08, 0x52, 0xa2, 0x6f,
            0x55, 0xae, 0x93, 0xb5, 0x9b, 0x46, 0x91, 0x3d, 0xdf, 0x89, 0x1c, 0xb5, 0x38, 0x75,
            0x5a, 0x45, 0x47, 0xde, 0x51, 0x12, 0x01, 0x5d, 0xc3, 0x00, 0x15, 0x7a, 0x1f, 0x37,
            0xe7, 0x14, 0x28, 0xbc, 0x04, 0x75, 0xd9, 0x26, 0x8e, 0x4b, 0x53, 0x06, 0xd3, 0xa1,
            0x22, 0x40, 0x13, 0xc4, 0x36, 0xbd, 0x1c, 0x23, 0xd7, 0x2a, 0x1c, 0x16, 0x46, 0x43,
            0x50, 0x00, 0x07, 0x48, 0xc7, 0x7c, 0x80, 0xbd, 0x11, 0xee, 0x41, 0x20, 0xb3, 0x52,
            0x3b, 0xd0, 0x0a, 0x2f, 0xdd, 0x02, 0xe8, 0xef, 0xfc, 0x7a, 0xfe, 0x3d, 0xd6, 0x73,
            0x2c, 0x5d, 0xa0, 0x6b, 0x08, 0x98, 0x6c,
        ];
        let msg = Message::Transfer {
            amount: Amount([
                0xfa, 0x22, 0x8b, 0x3d, 0xf3, 0x59, 0x21, 0xa1, 0xc9, 0x39, 0xad, 0x9c, 0x54, 0xe1,
                0x2f, 0x87, 0xc6, 0x27, 0x25, 0xd1, 0xaf, 0x7c, 0x7b, 0x6a, 0xea, 0xbf, 0x48, 0x14,
                0xe7, 0x26, 0x56, 0xfa,
            ]),
            token_address: Address([
                0xe8, 0xf2, 0xd2, 0x53, 0xd1, 0x08, 0x52, 0xa2, 0x6f, 0x55, 0xae, 0x93, 0xb5, 0x9b,
                0x46, 0x91, 0x3d, 0xdf, 0x89, 0x1c, 0xb5, 0x38, 0x75, 0x5a, 0x45, 0x47, 0xde, 0x51,
                0x12, 0x01, 0x5d, 0xc3,
            ]),
            token_chain: Chain::Sui,
            recipient: Address([
                0x7a, 0x1f, 0x37, 0xe7, 0x14, 0x28, 0xbc, 0x04, 0x75, 0xd9, 0x26, 0x8e, 0x4b, 0x53,
                0x06, 0xd3, 0xa1, 0x22, 0x40, 0x13, 0xc4, 0x36, 0xbd, 0x1c, 0x23, 0xd7, 0x2a, 0x1c,
                0x16, 0x46, 0x43, 0x50,
            ]),
            recipient_chain: Chain::Oasis,
            fee: Amount([
                0x48, 0xc7, 0x7c, 0x80, 0xbd, 0x11, 0xee, 0x41, 0x20, 0xb3, 0x52, 0x3b, 0xd0, 0x0a,
                0x2f, 0xdd, 0x02, 0xe8, 0xef, 0xfc, 0x7a, 0xfe, 0x3d, 0xd6, 0x73, 0x2c, 0x5d, 0xa0,
                0x6b, 0x08, 0x98, 0x6c,
            ]),
        };

        assert_eq!(payload.as_ref(), &serde_wormhole::to_vec(&msg).unwrap());
        assert_eq!(msg, serde_wormhole::from_slice(&payload).unwrap());
    }

    #[test]
    pub fn asset_meta() {
        let payload = [
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0xbe, 0xef, 0xfa, 0xce, 0x00, 0x02, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x45, 0x45, 0x46, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x42, 0x65, 0x65, 0x66, 0x20, 0x66, 0x61, 0x63, 0x65, 0x20, 0x54, 0x6f, 0x6b,
            0x65, 0x6e,
        ];

        let msg = Message::AssetMeta {
            token_address: Address([
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xbe, 0xef, 0xfa, 0xce,
            ]),
            token_chain: Chain::Ethereum,
            decimals: 12,
            symbol: "BEEF".into(),
            name: "Beef face Token".into(),
        };

        assert_eq!(payload.as_ref(), &serde_wormhole::to_vec(&msg).unwrap());
        assert_eq!(msg, serde_wormhole::from_slice(&payload).unwrap());
    }

    #[test]
    pub fn transfer_with_payload() {
        let payload = [
            0x03, 0x99, 0x30, 0x29, 0x01, 0x67, 0xf3, 0x48, 0x6a, 0xf6, 0x4c, 0xdd, 0x07, 0x64,
            0xa1, 0x6a, 0xca, 0xcc, 0xc5, 0x64, 0x2a, 0x66, 0x71, 0xaa, 0x87, 0x98, 0x64, 0x0c,
            0x25, 0x1f, 0x39, 0x73, 0x25, 0x12, 0x98, 0x15, 0xe5, 0x79, 0xdb, 0xd8, 0x25, 0xfd,
            0x72, 0x58, 0x99, 0x97, 0xd1, 0x78, 0xb6, 0x2a, 0x57, 0x47, 0xab, 0xcf, 0x51, 0x62,
            0xa5, 0xb7, 0xec, 0x19, 0x4e, 0x03, 0x51, 0x41, 0x18, 0x00, 0x08, 0xa4, 0x7e, 0xa2,
            0x49, 0xd4, 0xf0, 0x56, 0x6f, 0xbd, 0x12, 0x77, 0xfe, 0xa8, 0x6e, 0x92, 0x81, 0x3a,
            0x35, 0x90, 0x3b, 0x29, 0x73, 0x4d, 0x75, 0x56, 0x0a, 0xfe, 0x70, 0xb0, 0xb9, 0x10,
            0x33, 0x00, 0x14, 0x47, 0x3e, 0x51, 0x74, 0x32, 0xbc, 0x3f, 0xb3, 0xf9, 0x82, 0xbb,
            0xf3, 0xc9, 0x43, 0x41, 0xcf, 0x74, 0x24, 0xff, 0xa4, 0x02, 0x35, 0xbe, 0xb1, 0x7c,
            0x47, 0x16, 0xba, 0xbc, 0xaa, 0xbe, 0x99, 0x54, 0xa2, 0x97, 0xbe, 0x2a, 0xed, 0xae,
            0x91, 0x95, 0xb9, 0x6a, 0x8a, 0xba, 0xbd, 0x3a, 0xc1, 0xa4, 0xd1, 0x96, 0x0a, 0xf9,
            0xab, 0x92, 0x42, 0x0d, 0x41, 0x33, 0xe5, 0xa8, 0x37, 0x32, 0xd3, 0xc3, 0xb8, 0xf3,
            0x93, 0xd7, 0xc0, 0x9e, 0xe8, 0x87, 0xae, 0x16, 0xbf, 0xfa, 0x5e, 0x70, 0xea, 0x36,
            0xa2, 0x82, 0x37, 0x1d, 0x46, 0x81, 0x94, 0x10, 0x34, 0xb1, 0xad, 0x0f, 0x4b, 0xc9,
            0x17, 0x1e, 0x91, 0x25, 0x11,
        ];
        let msg = Message::TransferWithPayload {
            amount: Amount([
                0x99, 0x30, 0x29, 0x01, 0x67, 0xf3, 0x48, 0x6a, 0xf6, 0x4c, 0xdd, 0x07, 0x64, 0xa1,
                0x6a, 0xca, 0xcc, 0xc5, 0x64, 0x2a, 0x66, 0x71, 0xaa, 0x87, 0x98, 0x64, 0x0c, 0x25,
                0x1f, 0x39, 0x73, 0x25,
            ]),
            token_address: Address([
                0x12, 0x98, 0x15, 0xe5, 0x79, 0xdb, 0xd8, 0x25, 0xfd, 0x72, 0x58, 0x99, 0x97, 0xd1,
                0x78, 0xb6, 0x2a, 0x57, 0x47, 0xab, 0xcf, 0x51, 0x62, 0xa5, 0xb7, 0xec, 0x19, 0x4e,
                0x03, 0x51, 0x41, 0x18,
            ]),
            token_chain: Chain::Algorand,
            recipient: Address([
                0xa4, 0x7e, 0xa2, 0x49, 0xd4, 0xf0, 0x56, 0x6f, 0xbd, 0x12, 0x77, 0xfe, 0xa8, 0x6e,
                0x92, 0x81, 0x3a, 0x35, 0x90, 0x3b, 0x29, 0x73, 0x4d, 0x75, 0x56, 0x0a, 0xfe, 0x70,
                0xb0, 0xb9, 0x10, 0x33,
            ]),
            recipient_chain: Chain::Osmosis,
            sender_address: Address([
                0x47, 0x3e, 0x51, 0x74, 0x32, 0xbc, 0x3f, 0xb3, 0xf9, 0x82, 0xbb, 0xf3, 0xc9, 0x43,
                0x41, 0xcf, 0x74, 0x24, 0xff, 0xa4, 0x02, 0x35, 0xbe, 0xb1, 0x7c, 0x47, 0x16, 0xba,
                0xbc, 0xaa, 0xbe, 0x99,
            ]),
        };

        assert_eq!(&payload[..133], &serde_wormhole::to_vec(&msg).unwrap());
        let (actual, data) = serde_wormhole::from_slice_with_payload(&payload).unwrap();
        assert_eq!(msg, actual);
        assert_eq!(&payload[133..], data);
    }

    #[test]
    fn register_chain() {
        let buf = [
            0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0xb0, 0x72, 0x50, 0x5b, 0x5b, 0x99, 0x9c,
            0x1d, 0x08, 0x90, 0x5c, 0x02, 0xe2, 0xb6, 0xb2, 0x83, 0x2e, 0xf7, 0x2c, 0x0b, 0xa6,
            0xc8, 0xdb, 0x4f, 0x77, 0xfe, 0x45, 0x7e, 0xf2, 0xb3, 0xd0, 0x53, 0x41, 0x0b, 0x1e,
            0x92, 0xa9, 0x19, 0x4d, 0x92, 0x10, 0xdf, 0x24, 0xd9, 0x87, 0xac, 0x83, 0xd7, 0xb6,
            0xf0, 0xc2, 0x1c, 0xe9, 0x0f, 0x8b, 0xc1, 0x86, 0x9d, 0xe0, 0x89, 0x8b, 0xda, 0x7e,
            0x98, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x01, 0x3c, 0x1b, 0xfa, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x42, 0x72, 0x69, 0x64, 0x67,
            0x65, 0x01, 0x00, 0x00, 0x00, 0x01, 0x3b, 0x26, 0x40, 0x9f, 0x8a, 0xad, 0xed, 0x3f,
            0x5d, 0xdc, 0xa1, 0x84, 0x69, 0x5a, 0xa6, 0xa0, 0xfa, 0x82, 0x9b, 0x0c, 0x85, 0xca,
            0xf8, 0x48, 0x56, 0x32, 0x48, 0x96, 0xd2, 0x14, 0xca, 0x98,
        ];

        let vaa = Vaa {
            version: 1,
            guardian_set_index: 0,
            signatures: vec![Signature {
                index: 0,
                signature: [
                    0xb0, 0x72, 0x50, 0x5b, 0x5b, 0x99, 0x9c, 0x1d, 0x08, 0x90, 0x5c, 0x02, 0xe2,
                    0xb6, 0xb2, 0x83, 0x2e, 0xf7, 0x2c, 0x0b, 0xa6, 0xc8, 0xdb, 0x4f, 0x77, 0xfe,
                    0x45, 0x7e, 0xf2, 0xb3, 0xd0, 0x53, 0x41, 0x0b, 0x1e, 0x92, 0xa9, 0x19, 0x4d,
                    0x92, 0x10, 0xdf, 0x24, 0xd9, 0x87, 0xac, 0x83, 0xd7, 0xb6, 0xf0, 0xc2, 0x1c,
                    0xe9, 0x0f, 0x8b, 0xc1, 0x86, 0x9d, 0xe0, 0x89, 0x8b, 0xda, 0x7e, 0x98, 0x01,
                ],
            }],
            timestamp: 1,
            nonce: 1,
            emitter_chain: Chain::Solana,
            emitter_address: GOVERNANCE_EMITTER,
            sequence: 20_716_538,
            consistency_level: 0,
            payload: GovernancePacket {
                chain: Chain::Any,
                action: Action::RegisterChain {
                    chain: Chain::Solana,
                    emitter_address: Address([
                        0x3b, 0x26, 0x40, 0x9f, 0x8a, 0xad, 0xed, 0x3f, 0x5d, 0xdc, 0xa1, 0x84,
                        0x69, 0x5a, 0xa6, 0xa0, 0xfa, 0x82, 0x9b, 0x0c, 0x85, 0xca, 0xf8, 0x48,
                        0x56, 0x32, 0x48, 0x96, 0xd2, 0x14, 0xca, 0x98,
                    ]),
                },
            },
        };

        assert_eq!(buf.as_ref(), &serde_wormhole::to_vec(&vaa).unwrap());
        assert_eq!(vaa, serde_wormhole::from_slice(&buf).unwrap());
    }

    #[test]
    fn contract_upgrade() {
        let buf = [
            0x01, 0x00, 0x00, 0x00, 0x01, 0x0d, 0x02, 0x73, 0xaf, 0x3c, 0x2f, 0x55, 0x98, 0x90,
            0x41, 0xb0, 0x06, 0x8a, 0x4e, 0xae, 0xba, 0x8f, 0x88, 0x25, 0x10, 0x60, 0x21, 0x99,
            0x6d, 0xe6, 0x14, 0x39, 0x54, 0xc0, 0x6f, 0xc4, 0xcf, 0xad, 0x5c, 0x3f, 0x1a, 0xba,
            0x6c, 0xa9, 0x51, 0x18, 0x8e, 0x96, 0x2f, 0x11, 0x25, 0x1c, 0x25, 0xca, 0x98, 0x62,
            0x86, 0x85, 0xe2, 0xfc, 0x2b, 0x65, 0xec, 0x60, 0x94, 0x50, 0xcf, 0xc4, 0xe6, 0x51,
            0x12, 0x00, 0x03, 0x00, 0xc6, 0x2e, 0xb0, 0xde, 0x7a, 0xb3, 0xd0, 0x28, 0x92, 0x0f,
            0x18, 0xe1, 0x0d, 0xf7, 0xf2, 0xa0, 0x4d, 0x00, 0xeb, 0x88, 0xb8, 0x94, 0x0a, 0x27,
            0xbb, 0xce, 0x8c, 0xe6, 0x6c, 0x67, 0x52, 0x68, 0x82, 0xc9, 0x6c, 0xc5, 0x0d, 0xd3,
            0x81, 0xbd, 0xbf, 0xf3, 0x0b, 0xb6, 0x5b, 0x93, 0x47, 0xca, 0xdf, 0xf0, 0x96, 0x76,
            0x72, 0x90, 0x43, 0xaf, 0x60, 0xd6, 0x52, 0xa0, 0xb9, 0x06, 0x30, 0x00, 0x04, 0xf0,
            0x7b, 0x0f, 0xe6, 0xca, 0xb1, 0x8b, 0xe7, 0xae, 0x82, 0x7f, 0xe1, 0x1c, 0xa8, 0x99,
            0x2f, 0xf0, 0xb5, 0xa2, 0xa3, 0x2d, 0x65, 0xbe, 0x95, 0x82, 0x31, 0xbf, 0x84, 0xcf,
            0x6b, 0x14, 0xfb, 0x5e, 0xeb, 0x8a, 0x51, 0xad, 0xe1, 0xc7, 0x0a, 0xa5, 0xb8, 0xb8,
            0xcf, 0x83, 0xaf, 0xe3, 0xc2, 0x2d, 0x34, 0x71, 0x48, 0x1b, 0xda, 0x38, 0x96, 0xea,
            0x2a, 0x70, 0x46, 0x9f, 0x96, 0x92, 0xb8, 0x00, 0x05, 0x74, 0xd4, 0x96, 0x72, 0x9d,
            0xde, 0x6e, 0x19, 0x09, 0x59, 0xa1, 0x79, 0x45, 0xe8, 0x7c, 0xd9, 0x5e, 0x23, 0xc1,
            0x00, 0x47, 0xab, 0x2e, 0xef, 0x5c, 0x7d, 0x0e, 0xc6, 0xf9, 0x12, 0x08, 0xc7, 0x27,
            0x9d, 0x8a, 0x1b, 0x1f, 0x4a, 0x60, 0x06, 0x9d, 0x25, 0x93, 0x2a, 0x67, 0xd6, 0x55,
            0xf2, 0xd0, 0xb4, 0x20, 0x59, 0x00, 0x3d, 0xe7, 0x5f, 0xd3, 0xca, 0x79, 0x92, 0xfd,
            0xd1, 0xb0, 0xd4, 0x01, 0x06, 0x51, 0xf0, 0x6a, 0xb6, 0xfd, 0xfd, 0x42, 0xf3, 0x23,
            0x84, 0x0a, 0x81, 0x8b, 0x38, 0xcf, 0xd1, 0xc9, 0x9d, 0x22, 0x36, 0x6c, 0x87, 0x79,
            0x03, 0xb9, 0x8d, 0xf4, 0x0b, 0xda, 0xf2, 0xa2, 0x04, 0x2e, 0xb8, 0xcd, 0x0e, 0x59,
            0xc4, 0x63, 0x7e, 0x6a, 0x80, 0xa1, 0x99, 0xb1, 0x10, 0x8c, 0xd7, 0x65, 0x9b, 0xa8,
            0x37, 0x0f, 0x6f, 0x94, 0x76, 0xb4, 0x79, 0x83, 0x98, 0x54, 0xd8, 0xc1, 0xa6, 0x00,
            0x07, 0x78, 0x4b, 0x37, 0xc6, 0x10, 0x3c, 0x75, 0x2c, 0xd9, 0x7a, 0x58, 0x6b, 0xe8,
            0xe1, 0xce, 0xff, 0x22, 0xa6, 0xe4, 0x88, 0x43, 0x32, 0x84, 0xff, 0x31, 0xcd, 0x90,
            0x8b, 0x5c, 0x7d, 0x87, 0xe2, 0x44, 0xda, 0x75, 0x16, 0xd0, 0x7a, 0x0f, 0xa6, 0x92,
            0x68, 0x4f, 0x82, 0x6e, 0x5e, 0x6c, 0x8b, 0x45, 0x20, 0x04, 0x20, 0x6d, 0x6f, 0xe5,
            0xba, 0xe4, 0x6a, 0xfb, 0x1c, 0x21, 0x8c, 0xf5, 0x0d, 0x00, 0x09, 0x0b, 0x6c, 0xcc,
            0xe6, 0x7e, 0xd5, 0x01, 0xcb, 0x20, 0xfb, 0x06, 0xde, 0x76, 0xb9, 0x08, 0x3f, 0x13,
            0x29, 0xad, 0x78, 0xd3, 0x84, 0x51, 0x7a, 0x94, 0xab, 0x8e, 0x9a, 0xa6, 0x01, 0xbe,
            0x7a, 0x0f, 0x00, 0xbb, 0x60, 0x3b, 0x36, 0x50, 0x4a, 0x74, 0xdd, 0xc3, 0x67, 0x35,
            0x9c, 0x8e, 0xa0, 0x7f, 0x5c, 0xcd, 0x50, 0x22, 0x77, 0x2c, 0xc1, 0x57, 0xe0, 0x0f,
            0x54, 0x15, 0x63, 0x71, 0xe4, 0x01, 0x0b, 0xe1, 0xe6, 0xb7, 0x12, 0xa2, 0x35, 0x3e,
            0x49, 0x07, 0x92, 0x9c, 0x40, 0x57, 0xd5, 0xba, 0xbe, 0xf8, 0x0d, 0xc0, 0x9f, 0xeb,
            0xb3, 0xdc, 0x29, 0x34, 0xf4, 0x40, 0x48, 0x79, 0x5e, 0xef, 0x95, 0x4e, 0x99, 0x10,
            0x6c, 0xb2, 0x9b, 0x1a, 0x8d, 0x93, 0x53, 0x69, 0xda, 0xec, 0xbb, 0x8b, 0xae, 0x45,
            0x19, 0x2e, 0xfc, 0x3c, 0xed, 0xbc, 0x57, 0x31, 0x5d, 0x67, 0xec, 0x9e, 0xa8, 0x00,
            0x0a, 0x00, 0x0d, 0xdc, 0xc0, 0x4c, 0xd7, 0x55, 0x6a, 0x94, 0xe8, 0xb4, 0x18, 0xdc,
            0x3a, 0x8a, 0x00, 0xef, 0xb1, 0x28, 0xd4, 0xf7, 0x6f, 0x71, 0x43, 0x12, 0x47, 0xa9,
            0x2e, 0x7a, 0x19, 0xa8, 0x33, 0xfc, 0x02, 0x21, 0x5d, 0x1f, 0xd6, 0x9e, 0x0e, 0x59,
            0x6d, 0xa5, 0x31, 0xea, 0x58, 0x7a, 0xe2, 0xac, 0x2f, 0x36, 0xd2, 0x0f, 0x3b, 0x19,
            0x33, 0x06, 0x0a, 0xd6, 0xef, 0x3e, 0x9d, 0x7d, 0x84, 0x75, 0xba, 0x01, 0x0e, 0xc7,
            0x62, 0x16, 0x49, 0x49, 0x06, 0xae, 0xd8, 0xd8, 0x31, 0x0c, 0x31, 0xa5, 0xe4, 0xa5,
            0x45, 0xea, 0x2a, 0xf8, 0xdb, 0xe3, 0x8e, 0xeb, 0x74, 0xa3, 0xd1, 0x4f, 0x07, 0x34,
            0xdd, 0x2b, 0x1a, 0x21, 0xb9, 0x85, 0x0e, 0xa2, 0x0e, 0x6c, 0x2c, 0xa4, 0x22, 0x48,
            0x78, 0x8b, 0xfb, 0x17, 0x43, 0x9d, 0x37, 0xab, 0xda, 0x25, 0xd7, 0x05, 0xb5, 0x68,
            0xb2, 0x65, 0xb1, 0x13, 0xb9, 0x76, 0xc5, 0x00, 0x10, 0xc6, 0x1f, 0xe6, 0xeb, 0xea,
            0x61, 0xf2, 0xca, 0xe8, 0x95, 0xc3, 0x34, 0x96, 0x50, 0x70, 0x48, 0x7d, 0x39, 0xab,
            0x6b, 0xf0, 0x44, 0x28, 0x34, 0x0a, 0xf0, 0xf7, 0x69, 0x9f, 0xdd, 0xd3, 0xc0, 0x1e,
            0x0c, 0x86, 0xda, 0xea, 0x9e, 0xb4, 0x49, 0x70, 0x12, 0x48, 0xa2, 0x4f, 0xa4, 0xc0,
            0xff, 0x75, 0xfb, 0x60, 0x0b, 0x2c, 0x01, 0x34, 0xbd, 0x72, 0x17, 0x91, 0xf6, 0x67,
            0x11, 0x6e, 0x42, 0x00, 0x11, 0x19, 0xb7, 0x9c, 0xaa, 0x25, 0x0e, 0x75, 0xda, 0x14,
            0x82, 0xc7, 0xf4, 0x12, 0x68, 0x73, 0x08, 0xd1, 0x3e, 0xf7, 0x00, 0xf7, 0x26, 0xb9,
            0x94, 0x17, 0xbb, 0x75, 0xae, 0x6d, 0xd1, 0x43, 0xfc, 0x61, 0x9f, 0x8c, 0x9c, 0x29,
            0xc7, 0x3f, 0x99, 0x49, 0xaf, 0xfd, 0x16, 0x29, 0xcc, 0x28, 0x6a, 0x61, 0x91, 0x7e,
            0xd7, 0x85, 0x37, 0x54, 0xa4, 0x67, 0x02, 0x92, 0x0b, 0x76, 0xcf, 0x90, 0xeb, 0x00,
            0x12, 0xb0, 0xba, 0xb5, 0xe1, 0xa8, 0xb3, 0x21, 0xe9, 0x2f, 0x9d, 0x3d, 0xf9, 0x24,
            0x30, 0x18, 0x3e, 0x48, 0x43, 0xe5, 0x7c, 0x6f, 0x8c, 0x15, 0xc2, 0x2d, 0x62, 0xb2,
            0xf8, 0xe4, 0xb8, 0xcd, 0xc2, 0x75, 0x9f, 0x28, 0x54, 0xb8, 0xd1, 0x22, 0xac, 0xdb,
            0x4b, 0x4d, 0x89, 0x28, 0xb8, 0x7d, 0xf4, 0x19, 0x9c, 0xc6, 0x83, 0x01, 0x1d, 0x2e,
            0x9b, 0x7d, 0x41, 0xa9, 0x6d, 0x8b, 0x48, 0x0c, 0xcc, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x39, 0xc4, 0xba, 0x76, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x25, 0x64, 0xd2, 0xef,
            0xd6, 0x08, 0x4d, 0xf0, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x6f,
            0x6b, 0x65, 0x6e, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x02, 0x00, 0x03, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x09, 0x83,
        ];

        let vaa = Vaa {
            version: 1,
            guardian_set_index: 1,
            signatures: vec![
                Signature {
                    index: 2,
                    signature: [
                        0x73, 0xaf, 0x3c, 0x2f, 0x55, 0x98, 0x90, 0x41, 0xb0, 0x06, 0x8a, 0x4e,
                        0xae, 0xba, 0x8f, 0x88, 0x25, 0x10, 0x60, 0x21, 0x99, 0x6d, 0xe6, 0x14,
                        0x39, 0x54, 0xc0, 0x6f, 0xc4, 0xcf, 0xad, 0x5c, 0x3f, 0x1a, 0xba, 0x6c,
                        0xa9, 0x51, 0x18, 0x8e, 0x96, 0x2f, 0x11, 0x25, 0x1c, 0x25, 0xca, 0x98,
                        0x62, 0x86, 0x85, 0xe2, 0xfc, 0x2b, 0x65, 0xec, 0x60, 0x94, 0x50, 0xcf,
                        0xc4, 0xe6, 0x51, 0x12, 0x00,
                    ],
                },
                Signature {
                    index: 3,
                    signature: [
                        0x00, 0xc6, 0x2e, 0xb0, 0xde, 0x7a, 0xb3, 0xd0, 0x28, 0x92, 0x0f, 0x18,
                        0xe1, 0x0d, 0xf7, 0xf2, 0xa0, 0x4d, 0x00, 0xeb, 0x88, 0xb8, 0x94, 0x0a,
                        0x27, 0xbb, 0xce, 0x8c, 0xe6, 0x6c, 0x67, 0x52, 0x68, 0x82, 0xc9, 0x6c,
                        0xc5, 0x0d, 0xd3, 0x81, 0xbd, 0xbf, 0xf3, 0x0b, 0xb6, 0x5b, 0x93, 0x47,
                        0xca, 0xdf, 0xf0, 0x96, 0x76, 0x72, 0x90, 0x43, 0xaf, 0x60, 0xd6, 0x52,
                        0xa0, 0xb9, 0x06, 0x30, 0x00,
                    ],
                },
                Signature {
                    index: 4,
                    signature: [
                        0xf0, 0x7b, 0x0f, 0xe6, 0xca, 0xb1, 0x8b, 0xe7, 0xae, 0x82, 0x7f, 0xe1,
                        0x1c, 0xa8, 0x99, 0x2f, 0xf0, 0xb5, 0xa2, 0xa3, 0x2d, 0x65, 0xbe, 0x95,
                        0x82, 0x31, 0xbf, 0x84, 0xcf, 0x6b, 0x14, 0xfb, 0x5e, 0xeb, 0x8a, 0x51,
                        0xad, 0xe1, 0xc7, 0x0a, 0xa5, 0xb8, 0xb8, 0xcf, 0x83, 0xaf, 0xe3, 0xc2,
                        0x2d, 0x34, 0x71, 0x48, 0x1b, 0xda, 0x38, 0x96, 0xea, 0x2a, 0x70, 0x46,
                        0x9f, 0x96, 0x92, 0xb8, 0x00,
                    ],
                },
                Signature {
                    index: 5,
                    signature: [
                        0x74, 0xd4, 0x96, 0x72, 0x9d, 0xde, 0x6e, 0x19, 0x09, 0x59, 0xa1, 0x79,
                        0x45, 0xe8, 0x7c, 0xd9, 0x5e, 0x23, 0xc1, 0x00, 0x47, 0xab, 0x2e, 0xef,
                        0x5c, 0x7d, 0x0e, 0xc6, 0xf9, 0x12, 0x08, 0xc7, 0x27, 0x9d, 0x8a, 0x1b,
                        0x1f, 0x4a, 0x60, 0x06, 0x9d, 0x25, 0x93, 0x2a, 0x67, 0xd6, 0x55, 0xf2,
                        0xd0, 0xb4, 0x20, 0x59, 0x00, 0x3d, 0xe7, 0x5f, 0xd3, 0xca, 0x79, 0x92,
                        0xfd, 0xd1, 0xb0, 0xd4, 0x01,
                    ],
                },
                Signature {
                    index: 6,
                    signature: [
                        0x51, 0xf0, 0x6a, 0xb6, 0xfd, 0xfd, 0x42, 0xf3, 0x23, 0x84, 0x0a, 0x81,
                        0x8b, 0x38, 0xcf, 0xd1, 0xc9, 0x9d, 0x22, 0x36, 0x6c, 0x87, 0x79, 0x03,
                        0xb9, 0x8d, 0xf4, 0x0b, 0xda, 0xf2, 0xa2, 0x04, 0x2e, 0xb8, 0xcd, 0x0e,
                        0x59, 0xc4, 0x63, 0x7e, 0x6a, 0x80, 0xa1, 0x99, 0xb1, 0x10, 0x8c, 0xd7,
                        0x65, 0x9b, 0xa8, 0x37, 0x0f, 0x6f, 0x94, 0x76, 0xb4, 0x79, 0x83, 0x98,
                        0x54, 0xd8, 0xc1, 0xa6, 0x00,
                    ],
                },
                Signature {
                    index: 7,
                    signature: [
                        0x78, 0x4b, 0x37, 0xc6, 0x10, 0x3c, 0x75, 0x2c, 0xd9, 0x7a, 0x58, 0x6b,
                        0xe8, 0xe1, 0xce, 0xff, 0x22, 0xa6, 0xe4, 0x88, 0x43, 0x32, 0x84, 0xff,
                        0x31, 0xcd, 0x90, 0x8b, 0x5c, 0x7d, 0x87, 0xe2, 0x44, 0xda, 0x75, 0x16,
                        0xd0, 0x7a, 0x0f, 0xa6, 0x92, 0x68, 0x4f, 0x82, 0x6e, 0x5e, 0x6c, 0x8b,
                        0x45, 0x20, 0x04, 0x20, 0x6d, 0x6f, 0xe5, 0xba, 0xe4, 0x6a, 0xfb, 0x1c,
                        0x21, 0x8c, 0xf5, 0x0d, 0x00,
                    ],
                },
                Signature {
                    index: 9,
                    signature: [
                        0x0b, 0x6c, 0xcc, 0xe6, 0x7e, 0xd5, 0x01, 0xcb, 0x20, 0xfb, 0x06, 0xde,
                        0x76, 0xb9, 0x08, 0x3f, 0x13, 0x29, 0xad, 0x78, 0xd3, 0x84, 0x51, 0x7a,
                        0x94, 0xab, 0x8e, 0x9a, 0xa6, 0x01, 0xbe, 0x7a, 0x0f, 0x00, 0xbb, 0x60,
                        0x3b, 0x36, 0x50, 0x4a, 0x74, 0xdd, 0xc3, 0x67, 0x35, 0x9c, 0x8e, 0xa0,
                        0x7f, 0x5c, 0xcd, 0x50, 0x22, 0x77, 0x2c, 0xc1, 0x57, 0xe0, 0x0f, 0x54,
                        0x15, 0x63, 0x71, 0xe4, 0x01,
                    ],
                },
                Signature {
                    index: 11,
                    signature: [
                        0xe1, 0xe6, 0xb7, 0x12, 0xa2, 0x35, 0x3e, 0x49, 0x07, 0x92, 0x9c, 0x40,
                        0x57, 0xd5, 0xba, 0xbe, 0xf8, 0x0d, 0xc0, 0x9f, 0xeb, 0xb3, 0xdc, 0x29,
                        0x34, 0xf4, 0x40, 0x48, 0x79, 0x5e, 0xef, 0x95, 0x4e, 0x99, 0x10, 0x6c,
                        0xb2, 0x9b, 0x1a, 0x8d, 0x93, 0x53, 0x69, 0xda, 0xec, 0xbb, 0x8b, 0xae,
                        0x45, 0x19, 0x2e, 0xfc, 0x3c, 0xed, 0xbc, 0x57, 0x31, 0x5d, 0x67, 0xec,
                        0x9e, 0xa8, 0x00, 0x0a, 0x00,
                    ],
                },
                Signature {
                    index: 13,
                    signature: [
                        0xdc, 0xc0, 0x4c, 0xd7, 0x55, 0x6a, 0x94, 0xe8, 0xb4, 0x18, 0xdc, 0x3a,
                        0x8a, 0x00, 0xef, 0xb1, 0x28, 0xd4, 0xf7, 0x6f, 0x71, 0x43, 0x12, 0x47,
                        0xa9, 0x2e, 0x7a, 0x19, 0xa8, 0x33, 0xfc, 0x02, 0x21, 0x5d, 0x1f, 0xd6,
                        0x9e, 0x0e, 0x59, 0x6d, 0xa5, 0x31, 0xea, 0x58, 0x7a, 0xe2, 0xac, 0x2f,
                        0x36, 0xd2, 0x0f, 0x3b, 0x19, 0x33, 0x06, 0x0a, 0xd6, 0xef, 0x3e, 0x9d,
                        0x7d, 0x84, 0x75, 0xba, 0x01,
                    ],
                },
                Signature {
                    index: 14,
                    signature: [
                        0xc7, 0x62, 0x16, 0x49, 0x49, 0x06, 0xae, 0xd8, 0xd8, 0x31, 0x0c, 0x31,
                        0xa5, 0xe4, 0xa5, 0x45, 0xea, 0x2a, 0xf8, 0xdb, 0xe3, 0x8e, 0xeb, 0x74,
                        0xa3, 0xd1, 0x4f, 0x07, 0x34, 0xdd, 0x2b, 0x1a, 0x21, 0xb9, 0x85, 0x0e,
                        0xa2, 0x0e, 0x6c, 0x2c, 0xa4, 0x22, 0x48, 0x78, 0x8b, 0xfb, 0x17, 0x43,
                        0x9d, 0x37, 0xab, 0xda, 0x25, 0xd7, 0x05, 0xb5, 0x68, 0xb2, 0x65, 0xb1,
                        0x13, 0xb9, 0x76, 0xc5, 0x00,
                    ],
                },
                Signature {
                    index: 16,
                    signature: [
                        0xc6, 0x1f, 0xe6, 0xeb, 0xea, 0x61, 0xf2, 0xca, 0xe8, 0x95, 0xc3, 0x34,
                        0x96, 0x50, 0x70, 0x48, 0x7d, 0x39, 0xab, 0x6b, 0xf0, 0x44, 0x28, 0x34,
                        0x0a, 0xf0, 0xf7, 0x69, 0x9f, 0xdd, 0xd3, 0xc0, 0x1e, 0x0c, 0x86, 0xda,
                        0xea, 0x9e, 0xb4, 0x49, 0x70, 0x12, 0x48, 0xa2, 0x4f, 0xa4, 0xc0, 0xff,
                        0x75, 0xfb, 0x60, 0x0b, 0x2c, 0x01, 0x34, 0xbd, 0x72, 0x17, 0x91, 0xf6,
                        0x67, 0x11, 0x6e, 0x42, 0x00,
                    ],
                },
                Signature {
                    index: 17,
                    signature: [
                        0x19, 0xb7, 0x9c, 0xaa, 0x25, 0x0e, 0x75, 0xda, 0x14, 0x82, 0xc7, 0xf4,
                        0x12, 0x68, 0x73, 0x08, 0xd1, 0x3e, 0xf7, 0x00, 0xf7, 0x26, 0xb9, 0x94,
                        0x17, 0xbb, 0x75, 0xae, 0x6d, 0xd1, 0x43, 0xfc, 0x61, 0x9f, 0x8c, 0x9c,
                        0x29, 0xc7, 0x3f, 0x99, 0x49, 0xaf, 0xfd, 0x16, 0x29, 0xcc, 0x28, 0x6a,
                        0x61, 0x91, 0x7e, 0xd7, 0x85, 0x37, 0x54, 0xa4, 0x67, 0x02, 0x92, 0x0b,
                        0x76, 0xcf, 0x90, 0xeb, 0x00,
                    ],
                },
                Signature {
                    index: 18,
                    signature: [
                        0xb0, 0xba, 0xb5, 0xe1, 0xa8, 0xb3, 0x21, 0xe9, 0x2f, 0x9d, 0x3d, 0xf9,
                        0x24, 0x30, 0x18, 0x3e, 0x48, 0x43, 0xe5, 0x7c, 0x6f, 0x8c, 0x15, 0xc2,
                        0x2d, 0x62, 0xb2, 0xf8, 0xe4, 0xb8, 0xcd, 0xc2, 0x75, 0x9f, 0x28, 0x54,
                        0xb8, 0xd1, 0x22, 0xac, 0xdb, 0x4b, 0x4d, 0x89, 0x28, 0xb8, 0x7d, 0xf4,
                        0x19, 0x9c, 0xc6, 0x83, 0x01, 0x1d, 0x2e, 0x9b, 0x7d, 0x41, 0xa9, 0x6d,
                        0x8b, 0x48, 0x0c, 0xcc, 0x01,
                    ],
                },
            ],
            timestamp: 0,
            nonce: 969_194_102,
            emitter_chain: Chain::Solana,
            emitter_address: GOVERNANCE_EMITTER,
            sequence: 2_694_510_404_604_284_400,
            consistency_level: 32,
            payload: GovernancePacket {
                chain: Chain::Terra,
                action: Action::ContractUpgrade {
                    new_contract: Address([
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x83,
                    ]),
                },
            },
        };

        assert_eq!(buf.as_ref(), &serde_wormhole::to_vec(&vaa).unwrap());
        assert_eq!(vaa, serde_wormhole::from_slice(&buf).unwrap());
    }
}
