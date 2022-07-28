mod key_share;
mod signature_algorithms;
mod supported_groups;
mod supported_version;

use std::ops::Deref;

use key_share::KeyShareEntries;
use signature_algorithms::SignatureAlgorithmList;
use supported_groups::SupportedGroupList;
use supported_version::SupportedVersionList;

use super::utils::Encode;

pub struct Extensions(Vec<ExtensionType>);

impl Encode for Extensions {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sub: Vec<u8> = Vec::new();
        self.iter().for_each(|ext| {
            ext.as_assigned_u16().encode(&mut sub);

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

pub enum ExtensionType {
    SupportedGroups(SupportedGroupList),
    SignatureAlgorithms(SignatureAlgorithmList),
    SupportedVersions(SupportedVersionList),
    KeyShare(KeyShareEntries),
}

impl ExtensionType {
    fn as_assigned_u16(&self) -> u16 {
        match self {
            Self::SupportedGroups(_) => 0x000a,
            Self::SignatureAlgorithms(_) => 0x000d,
            Self::SupportedVersions(_) => 0x002d,
            Self::KeyShare(_) => 0x0033,
        }
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

        assert_eq!(encoded, vec![]);
    }
}
