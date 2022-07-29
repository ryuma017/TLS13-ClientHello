use std::ops::Deref;

use crate::client_hello::{utils::{Encode, AssignedValue}, enums::NamedGroup};

pub struct SupportedGroupList(Vec<NamedGroup>);

impl Encode for SupportedGroupList {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sub: Vec<u8> = Vec::new();
        encode_supported_groups(&mut sub, self);
        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }
}

impl Default for SupportedGroupList {
    fn default() -> Self {
        use NamedGroup::*;

        Self(vec![
            secp256r1, secp384r1, secp521r1, X25519, X448, FFDHE2048, FFDHE3072, FFDHE4096,
            FFDHE6144, FFDHE8192,
        ])
    }
}

impl Deref for SupportedGroupList {
    type Target = Vec<NamedGroup>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn encode_supported_groups(bytes: &mut Vec<u8>, values: &[NamedGroup]) {
    let mut sub: Vec<u8> = Vec::new();
    values.iter().for_each(|sg| {
        sg.assigned_value().encode(&mut sub);
    });

    (sub.len() as u16).encode(bytes);
    bytes.append(&mut sub);
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::Encode;

    use super::SupportedGroupList;

    #[test]
    fn supported_groups_encoding_works() {
        let supported_groups = SupportedGroupList::default();
        let encoded = supported_groups.get_encoded_bytes();

        assert_eq!(
            encoded,
            vec![
                0x00, 0x16, 0x00, 0x14, 0x00, 0x17, 0x00, 0x18, 0x00, 0x19, 0x00, 0x1d, 0x00, 0x1e,
                0x01, 0x00, 0x01, 0x01, 0x01, 0x02, 0x01, 0x03, 0x01, 0x04
            ]
        )
    }
}
