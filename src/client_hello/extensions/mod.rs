mod key_share;
mod signature_algorithms;
mod supported_groups;
mod supported_versions;

use std::ops::Deref;

pub use key_share::KeyShareEntries;
pub use signature_algorithms::SignatureAlgorithmList;
pub use supported_groups::SupportedGroupList;
pub use supported_versions::SupportedVersionList;

use super::{utils::{Encode, AssignedValue}, enums::ExtensionType};

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

// #[cfg(test)]
// mod tests {
//     use crate::client_hello::utils::Encode;

//     use super::Extensions;

//     #[test]
//     fn extensions_encoding_works() {
//         let extensions = Extensions::default();
//         let encoded = extensions.get_encoded_bytes();

//         assert_eq!(encoded, vec![]);
//     }
// }
