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
