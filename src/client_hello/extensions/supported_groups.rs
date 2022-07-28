use std::ops::Deref;

use crate::client_hello::utils::Encode;

pub struct SupportedGroupList(Vec<SupportedGroup>);

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
        use SupportedGroup::*;

        Self(vec![
            secp256r1, secp384r1, secp521r1, X25519, X448, FFDHE2048, FFDHE3072, FFDHE4096,
            FFDHE6144, FFDHE8192,
        ])
    }
}

impl Deref for SupportedGroupList {
    type Target = Vec<SupportedGroup>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn encode_supported_groups(bytes: &mut Vec<u8>, values: &[SupportedGroup]) {
    let mut sub: Vec<u8> = Vec::new();
    values.iter().for_each(|sg| {
        sg.as_assigned_u16().encode(&mut sub);
    });

    (sub.len() as u16).encode(bytes);
    bytes.append(&mut sub);
}

#[allow(non_camel_case_types)]
pub enum SupportedGroup {
    secp256r1,
    secp384r1,
    secp521r1,
    X25519,
    X448,
    FFDHE2048,
    FFDHE3072,
    FFDHE4096,
    FFDHE6144,
    FFDHE8192,
}

impl SupportedGroup {
    fn as_assigned_u16(&self) -> u16 {
        match self {
            Self::secp256r1 => 0x0017,
            Self::secp384r1 => 0x0018,
            Self::secp521r1 => 0x0019,
            Self::X25519 => 0x001d,
            Self::X448 => 0x001e,
            Self::FFDHE2048 => 0x0100,
            Self::FFDHE3072 => 0x0101,
            Self::FFDHE4096 => 0x0102,
            Self::FFDHE6144 => 0x0103,
            Self::FFDHE8192 => 0x0104,
        }
    }
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
