use std::ops::Deref;

use super::utils::Encode;

/// In TLS 1.3 the list of possible cipher suites has been greatly reduced.
/// All the remaining suites are AEAD algorithms which provide stronger encryption
/// guarantees than many previous suites with an easier all-in-one implementation.
pub struct CipherSuites(Vec<CipherSuite>);

impl Encode for CipherSuites {
    fn encode(&self, bytes: &mut Vec<u8>) {
        (self.len() as u16).encode(bytes);
        encode_cipher_suites(bytes, self);
    }
}

fn encode_cipher_suites(bytes: &mut Vec<u8>, values: &[CipherSuite]) {
    values.iter().for_each(|cs| {
        cs.as_assigned_u16().encode(bytes);
    })
}

impl Default for CipherSuites {
    fn default() -> Self {
        Self(vec![CipherSuite::TLS13_AES_128_GCM_SHA256])
    }
}

impl Deref for CipherSuites {
    type Target = Vec<CipherSuite>;

    fn deref(&self) -> &Self::Target {
        &self.0
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

impl CipherSuite {
    /// Returns `u16` assigned to each suite.
    fn as_assigned_u16(&self) -> u16 {
        match self {
            Self::TLS13_AES_128_GCM_SHA256 => 0x1301,
            Self::TLS13_AES_256_GCM_SHA384 => 0x1302,
            Self::TLS13_CHACHA20_POLY1305_SHA256 => 0x1303,
            Self::TLS13_AES_128_CCM_SHA256 => 0x1304,
            Self::TLS13_AES_128_CCM_8_SHA256 => 0x1305,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::Encode;

    use super::CipherSuites;

    #[test]
    fn default_cipher_suites_encoding_works() {
        let cipher_suites = CipherSuites::default();
        let encoded = cipher_suites.get_encoded_bytes();

        assert_eq!(encoded, [0x00, 0x02, 0x13, 0x01]);
    }
}
