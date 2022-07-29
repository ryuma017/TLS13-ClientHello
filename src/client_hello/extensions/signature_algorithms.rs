use std::ops::Deref;

use crate::client_hello::{utils::{Encode, AssignedValue}, enums::SignatureScheme};

pub struct SignatureAlgorithmList(Vec<SignatureScheme>);

impl Encode for SignatureAlgorithmList {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sub: Vec<u8> = Vec::new();
        encode_signature_schemes(&mut sub, self);
        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }
}

impl Default for SignatureAlgorithmList {
    fn default() -> Self {
        use SignatureScheme::*;

        Self(vec![
            ECDSA_SECP256r1_SHA256,
            ECDSA_SECP384r1_SHA384,
            ECDSA_SECP521r1_SHA512,
        ])
    }
}

impl Deref for SignatureAlgorithmList {
    type Target = Vec<SignatureScheme>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn encode_signature_schemes(bytes: &mut Vec<u8>, values: &[SignatureScheme]) {
    let mut sub: Vec<u8> = Vec::new();
    values.iter().for_each(|ss| {
        ss.assigned_value().encode(&mut sub);
    });

    (sub.len() as u16).encode(bytes);
    bytes.append(&mut sub);
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::Encode;

    use super::SignatureAlgorithmList;

    #[test]
    fn signature_algorithms_encoding_works() {
        let signature_algorithms = SignatureAlgorithmList::default();
        let encoded = signature_algorithms.get_encoded_bytes();

        assert_eq!(
            encoded,
            vec![0x00, 0x08, 0x00, 0x06, 0x04, 0x03, 0x05, 0x03, 0x06, 0x03]
        )
    }
}
