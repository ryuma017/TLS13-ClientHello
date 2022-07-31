use crate::client_hello::{
    extensions::{
        KeyShareEntries, SignatureAlgorithmList, SupportedGroupList, SupportedVersionList,
    },
    utils::AssignedValue,
};

use super::{ClientHello, utils::Encode};

pub enum ProtocolVersion {
    TLSv1_0,
    TLSv1_1,
    TLSv1_2,
    TLSv1_3,
}

impl AssignedValue for ProtocolVersion {
    type Integer = u16;

    fn assigned_value(&self) -> Self::Integer {
        match *self {
            Self::TLSv1_0 => 0x0301,
            Self::TLSv1_1 => 0x0302,
            Self::TLSv1_2 => 0x0303,
            Self::TLSv1_3 => 0x0304,
        }
    }
}

/// The possible cipher suites enum in TLS 1.3.
///
/// All the suites are AEAD (Authenticated Encryption with Associated Data) algorithms.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum CipherSuite {
    TLS13_AES_128_GCM_SHA256,
    TLS13_AES_256_GCM_SHA384,
    TLS13_CHACHA20_POLY1305_SHA256,
    TLS13_AES_128_CCM_SHA256,
    TLS13_AES_128_CCM_8_SHA256,
}

impl AssignedValue for CipherSuite {
    type Integer = u16;

    fn assigned_value(&self) -> Self::Integer {
        match *self {
            Self::TLS13_AES_128_GCM_SHA256 => 0x1301,
            Self::TLS13_AES_256_GCM_SHA384 => 0x1302,
            Self::TLS13_CHACHA20_POLY1305_SHA256 => 0x1303,
            Self::TLS13_AES_128_CCM_SHA256 => 0x1304,
            Self::TLS13_AES_128_CCM_8_SHA256 => 0x1305,
        }
    }
}

/// The compression TLS protocol enum.
///
/// In this implementation, only `Null` is used.
/// Methods other than `Null` are just added for backward compatibility.
#[allow(clippy::upper_case_acronyms)]
pub enum CompressionMethod {
    Null,
    Deflate, // Never used.
    LSZ,     // Never used.
}

impl AssignedValue for CompressionMethod {
    type Integer = u8;

    fn assigned_value(&self) -> Self::Integer {
        match *self {
            Self::Null => 0x00,
            Self::Deflate => 0x01,
            Self::LSZ => 0x40,
        }
    }
}

pub enum ExtensionType {
    SupportedGroups(SupportedGroupList),
    SignatureAlgorithms(SignatureAlgorithmList),
    SupportedVersions(SupportedVersionList),
    KeyShare(KeyShareEntries),
}

impl AssignedValue for ExtensionType {
    type Integer = u16;

    fn assigned_value(&self) -> Self::Integer {
        match *self {
            Self::SupportedGroups(_) => 0x000a,
            Self::SignatureAlgorithms(_) => 0x000d,
            Self::SupportedVersions(_) => 0x002d,
            Self::KeyShare(_) => 0x0033,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum NamedGroup {
    secp256r1,
    secp384r1,
    secp521r1,
    X25519,
    X448,
    FFDHE2048,
    FFDHE3072,
    FFDHE4096,
    FFDHE6144,
    FFDHE8192,
}

impl AssignedValue for NamedGroup {
    type Integer = u16;

    fn assigned_value(&self) -> Self::Integer {
        match *self {
            Self::secp256r1 => 0x0017,
            Self::secp384r1 => 0x0018,
            Self::secp521r1 => 0x0019,
            Self::X25519 => 0x001d,
            Self::X448 => 0x001e,
            Self::FFDHE2048 => 0x0100,
            Self::FFDHE3072 => 0x0101,
            Self::FFDHE4096 => 0x0102,
            Self::FFDHE6144 => 0x0103,
            Self::FFDHE8192 => 0x0104,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum SignatureScheme {
    ECDSA_SECP256r1_SHA256,
    ECDSA_SECP384r1_SHA384,
    ECDSA_SECP521r1_SHA512,
    ED25519,
    ED448,
    RSA_PSS_PSS_SHA256,
    RSA_PSS_PSS_SHA384,
    RSA_PSS_PSS_SHA512,
    RSA_PSS_RSAE_SHA256,
    RSA_PSS_RSAE_SHA384,
    RSA_PSS_RSAE_SHA512,
    RSA_PKCS1_SHA256,
    RSA_PKCS1_SHA384,
    RSA_PKCS1_SHA512,
}

impl AssignedValue for SignatureScheme {
    type Integer = u16;

    fn assigned_value(&self) -> Self::Integer {
        match *self {
            Self::ECDSA_SECP256r1_SHA256 => 0x0403,
            Self::ECDSA_SECP384r1_SHA384 => 0x0503,
            Self::ECDSA_SECP521r1_SHA512 => 0x0603,
            Self::ED25519 => 0x0807,
            Self::ED448 => 0x0808,
            Self::RSA_PSS_PSS_SHA256 => 0x0809,
            Self::RSA_PSS_PSS_SHA384 => 0x080a,
            Self::RSA_PSS_PSS_SHA512 => 0x080b,
            Self::RSA_PSS_RSAE_SHA256 => 0x0804,
            Self::RSA_PSS_RSAE_SHA384 => 0x0805,
            Self::RSA_PSS_RSAE_SHA512 => 0x0806,
            Self::RSA_PKCS1_SHA256 => 0x0401,
            Self::RSA_PKCS1_SHA384 => 0x0501,
            Self::RSA_PKCS1_SHA512 => 0x0601,
        }
    }
}

// とりあえず ClientHello だけ
// TODO: 気が向いたら拡張していきたいなぁ
pub enum HandshakeType {
    ClientHello
}

impl AssignedValue for HandshakeType {
    type Integer = u8;

    fn assigned_value(&self) -> Self::Integer {
        match *self {
            Self::ClientHello => 0x01,
        }
    }
}

pub enum HandshakeData {
    ClientHello(ClientHello)
}

impl HandshakeData {
    pub fn encode(&self, bytes: &mut Vec<u8>) {
        match *self {
            Self::ClientHello(ref data) => data.encode(bytes),
        }
    }
}
