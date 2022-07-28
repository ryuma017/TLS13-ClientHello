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
