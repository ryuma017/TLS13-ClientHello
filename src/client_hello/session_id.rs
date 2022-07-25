use super::utils::{Encode, Legacy};

/// In TLS 1.3, a value in this field is used to trigger "middlebox compatibility mode"
/// which helps TLS 1.3 sessions to be disguised as resumed TLS 1.2 sessions.
pub struct SessionID {
    length: usize,
    value: [u8; 32],
}

impl Encode for SessionID {
    fn encode(&self, bytes: &mut Vec<u8>) {
        (self.length as u8).encode(bytes);
        self.value[..self.length].encode(bytes);
    }
}

impl Legacy for SessionID {
    /// In this implementation of TLS 1.3 Client Hello makes random session id predictable.
    fn legacy() -> Self {
        let mut session_id = [0u8; 32];
        fill_with_fake_session_id(&mut session_id);

        Self {
            length: session_id.len(),
            value: session_id,
        }
    }
}

/// This is fake function.
///
/// Fills the given `u8` array with a predictable session id.
fn fill_with_fake_session_id(bytes: &mut [u8; 32]) {
    const FAKE_SESSION_ID: [u8; 32] = [
        0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee,
        0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd,
        0xfe, 0xff,
    ];

    bytes.copy_from_slice(&FAKE_SESSION_ID);
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::{Encode, Legacy};

    use super::SessionID;

    #[test]
    fn session_id_encoding_works() {
        let session_id = SessionID::legacy();
        let encoded = session_id.get_encoded_bytes();

        assert_eq!(
            encoded,
            // 0x20             - 0x20 (32) bytes of session ID follow
            // 0xe0 0xe1 … 0xff - fake session ID
            [
                0x20, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec,
                0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa,
                0xfb, 0xfc, 0xfd, 0xfe, 0xff
            ]
        )
    }
}
