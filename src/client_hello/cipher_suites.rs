use std::ops::Deref;

use super::{utils::{Encode, AssignedValue}, enums::CipherSuite};

/// In TLS 1.3 the list of possible cipher suites has been greatly reduced.
/// All the remaining suites are AEAD algorithms which provide stronger encryption
/// guarantees than many previous suites with an easier all-in-one implementation.
pub struct CipherSuites(Vec<CipherSuite>);

impl Encode for CipherSuites {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sub = Vec::new();
        encode_cipher_suites(&mut sub, self);
        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }
}

fn encode_cipher_suites(bytes: &mut Vec<u8>, values: &[CipherSuite]) {
    values.iter().for_each(|cs| {
        cs.assigned_value().encode(bytes);
    })
}

impl Default for CipherSuites {
    fn default() -> Self {
        use CipherSuite::*;

        Self(vec![TLS13_AES_128_GCM_SHA256])
    }
}

impl Deref for CipherSuites {
    type Target = Vec<CipherSuite>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
