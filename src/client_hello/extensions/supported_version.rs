use std::ops::Deref;

use crate::client_hello::utils::Encode;

pub struct SupportedVersionList(Vec<SupportedVersion>);

impl Encode for SupportedVersionList {
    fn encode(&self, bytes: &mut Vec<u8>) {
        todo!()
    }
}

impl Default for SupportedVersionList {
    fn default() -> Self {
        todo!()
    }
}

impl Deref for SupportedVersionList {
    type Target = Vec<SupportedVersion>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum SupportedVersion {}
