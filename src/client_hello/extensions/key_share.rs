use std::ops::Deref;

use crate::client_hello::{
    enums::NamedGroup,
    utils::{AssignedValue, Encode},
};

/// Genarate with openssl command.
///
/// ```sh
/// $ openssl genpkey -algorithm ED25519 -out private.key
/// $ openssl pkey -noout -text < private.key
/// ED25519 Private-Key:
/// priv:
///     [...]
/// pub:
///     21:01:9f:6f:71:56:e6:ab:69:2c:6c:ed:1a:af:a6:
///     31:2f:3e:a5:b9:e0:16:64:39:cb:73:06:bd:85:64:
///     7e:9a
/// ```
const KEY_EXCHANGE: [u8; 32] = [
    0x21, 0x01, 0x9f, 0x6f, 0x71, 0x56, 0xe6, 0xab, 0x69, 0x2c, 0x6c, 0xed, 0x1a, 0xaf, 0xa6, 0x31,
    0x2f, 0x3e, 0xa5, 0xb9, 0xe0, 0x16, 0x64, 0x39, 0xcb, 0x73, 0x06, 0xbd, 0x85, 0x64, 0x7e, 0x9a,
];

pub struct KeyShareEntries(Vec<KeyShareEntry>);

impl Encode for KeyShareEntries {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sub: Vec<u8> = Vec::new();
        encode_key_share_entries(&mut sub, self);
        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }
}

fn encode_key_share_entries(bytes: &mut Vec<u8>, values: &[KeyShareEntry]) {
    let mut sub: Vec<u8> = Vec::new();
    values.iter().for_each(|ks| {
        ks.encode(&mut sub);
    });

    (sub.len() as u16).encode(bytes);
    bytes.append(&mut sub);
}

impl Default for KeyShareEntries {
    fn default() -> Self {
        Self(vec![KeyShareEntry::default()])
    }
}

impl Deref for KeyShareEntries {
    type Target = Vec<KeyShareEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct KeyShareEntry {
    group: NamedGroup,
    data: Vec<u8>,
}

impl Encode for KeyShareEntry {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.group.assigned_value().encode(bytes);
        (self.data.len() as u16).encode(bytes);
        bytes.extend_from_slice(&self.data);
    }
}

impl Default for KeyShareEntry {
    fn default() -> Self {
        Self {
            group: NamedGroup::X25519,
            data: KEY_EXCHANGE.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::Encode;

    use super::KeyShareEntries;

    #[test]
    fn key_share_entries_encoding_works() {
        let key_share_entries = KeyShareEntries::default();
        let encoded = key_share_entries.get_encoded_bytes();

        assert_eq!(
            encoded,
            vec![
                0x00, 0x26, 0x00, 0x24, 0x00, 0x1d, 0x00, 0x20, 0x21, 0x01, 0x9f, 0x6f, 0x71, 0x56,
                0xe6, 0xab, 0x69, 0x2c, 0x6c, 0xed, 0x1a, 0xaf, 0xa6, 0x31, 0x2f, 0x3e, 0xa5, 0xb9,
                0xe0, 0x16, 0x64, 0x39, 0xcb, 0x73, 0x06, 0xbd, 0x85, 0x64, 0x7e, 0x9a
            ]
        )
    }
}
