use std::ops::Deref;

use crate::client_hello::utils::Encode;

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
        ss.as_assigned_u16().encode(&mut sub);
    });

    (sub.len() as u16).encode(bytes);
    bytes.append(&mut sub);
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

impl SignatureScheme {
    fn as_assigned_u16(&self) -> u16 {
        match self {
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
