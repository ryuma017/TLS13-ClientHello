use std::ops::Deref;

use crate::client_hello::utils::Encode;

pub struct KeyShareEntries(Vec<KeyShare>);

impl Encode for KeyShareEntries {
    fn encode(&self, bytes: &mut Vec<u8>) {
        todo!()
    }
}

impl Default for KeyShareEntries {
    fn default() -> Self {
        todo!()
    }
}

impl Deref for KeyShareEntries {
    type Target = Vec<KeyShare>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum KeyShare {}
