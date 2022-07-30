mod key_share;
mod signature_algorithms;
mod supported_groups;
mod supported_versions;

use std::ops::Deref;

pub use key_share::KeyShareEntries;
pub use signature_algorithms::SignatureAlgorithmList;
pub use supported_groups::SupportedGroupList;
pub use supported_versions::SupportedVersionList;

use super::{
    enums::ExtensionType,
    utils::{AssignedValue, Encode},
};

pub struct Extensions(Vec<ExtensionType>);

impl Encode for Extensions {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sub: Vec<u8> = Vec::new();
        self.iter().for_each(|ext| {
            ext.assigned_value().encode(&mut sub);

            match ext {
                ExtensionType::SupportedGroups(ref e) => e.encode(&mut sub),
                ExtensionType::SignatureAlgorithms(ref e) => e.encode(&mut sub),
                ExtensionType::SupportedVersions(ref e) => e.encode(&mut sub),
                ExtensionType::KeyShare(ref e) => e.encode(&mut sub),
            };
        });

        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }
}

impl Default for Extensions {
    fn default() -> Self {
        use ExtensionType::*;

        Self(vec![
            SupportedGroups(SupportedGroupList::default()),
            SignatureAlgorithms(SignatureAlgorithmList::default()),
            SupportedVersions(SupportedVersionList::default()),
            KeyShare(KeyShareEntries::default()),
        ])
    }
}

impl Deref for Extensions {
    type Target = Vec<ExtensionType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::Encode;

    use super::Extensions;

    #[test]
    fn extensions_encoding_works() {
        let extensions = Extensions::default();
        let encoded = extensions.get_encoded_bytes();

        assert_eq!(
            encoded,
            vec![
                0x00, 0x57, 0x00, 0x0a, 0x00, 0x16, 0x00, 0x14, 0x00, 0x17, 0x00, 0x18, 0x00, 0x19,
                0x00, 0x1d, 0x00, 0x1e, 0x01, 0x00, 0x01, 0x01, 0x01, 0x02, 0x01, 0x03, 0x01, 0x04,
                0x00, 0x0d, 0x00, 0x08, 0x00, 0x06, 0x04, 0x03, 0x05, 0x03, 0x06, 0x03, 0x00, 0x2d,
                0x00, 0x03, 0x02, 0x03, 0x04, 0x00, 0x33, 0x00, 0x26, 0x00, 0x24, 0x00, 0x1d, 0x00,
                0x20, 0x21, 0x01, 0x9f, 0x6f, 0x71, 0x56, 0xe6, 0xab, 0x69, 0x2c, 0x6c, 0xed, 0x1a,
                0xaf, 0xa6, 0x31, 0x2f, 0x3e, 0xa5, 0xb9, 0xe0, 0x16, 0x64, 0x39, 0xcb, 0x73, 0x06,
                0xbd, 0x85, 0x64, 0x7e, 0x9a
            ]
        );
    }
}
