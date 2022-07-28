use std::ops::Deref;

use crate::client_hello::utils::Encode;

pub struct SignatureAlgorithmList(Vec<SignatureAlgorithm>);

impl Encode for SignatureAlgorithmList {
    fn encode(&self, bytes: &mut Vec<u8>) {
        todo!()
    }
}

impl Default for SignatureAlgorithmList {
    fn default() -> Self {
        todo!()
    }
}

impl Deref for SignatureAlgorithmList {
    type Target = Vec<SignatureAlgorithm>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum SignatureAlgorithm {}

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
            vec![
                0x00, 0x08, 0x00, 0x06, 0x04, 0x03, 0x05, 0x03, 0x06, 0x03
            ]
        )
    }
}