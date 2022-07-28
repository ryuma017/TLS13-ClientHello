use std::ops::Deref;

use crate::client_hello::utils::Encode;

pub struct SupportedGroupList(Vec<SupportedGroup>);

impl Encode for SupportedGroupList {
    fn encode(&self, bytes: &mut Vec<u8>) {
        todo!()
    }
}

impl Default for SupportedGroupList {
    fn default() -> Self {
        todo!()
    }
}

impl Deref for SupportedGroupList {
    type Target = Vec<SupportedGroup>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum SupportedGroup {}
