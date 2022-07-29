use std::ops::Deref;

use crate::client_hello::{utils::{Encode, AssignedValue}, enums::ProtocolVersion};

pub struct SupportedVersionList(Vec<ProtocolVersion>);

impl Encode for SupportedVersionList {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sub: Vec<u8> = Vec::new();
        encode_protocol_versions(&mut sub, self);
        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }
}

impl Default for SupportedVersionList {
    fn default() -> Self {
        use ProtocolVersion::*;

        Self(vec![TLSv1_3])
    }
}

impl Deref for SupportedVersionList {
    type Target = Vec<ProtocolVersion>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn encode_protocol_versions(bytes: &mut Vec<u8>, values: &[ProtocolVersion]) {
    let mut sub: Vec<u8> = Vec::new();
    values.iter().for_each(|sg| {
        sg.assigned_value().encode(&mut sub);
    });

    (sub.len() as u8).encode(bytes);
    bytes.append(&mut sub);
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::Encode;

    use super::SupportedVersionList;

    #[test]
    fn supported_versions_encoding_works() {
        let supported_versions = SupportedVersionList::default();
        let encoded = supported_versions.get_encoded_bytes();

        assert_eq!(
            encoded,
            vec![0x00, 0x03, 0x02, 0x03, 0x04]
        )
    }
}
