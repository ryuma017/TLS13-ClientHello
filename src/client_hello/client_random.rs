use super::Encode;

/// The client provides 32 bytes of random data.
/// This data will be used later in the session.
pub struct ClientRandom([u8; 32]);

impl ClientRandom {
    /// In this implementation of TLS 1.3 Client Hello makes random data predictable.
    pub fn new() -> Self {
        let mut random_bytes = [0u8; 32];
        fill_random(&mut random_bytes);

        Self(random_bytes)
    }
}

impl Encode for ClientRandom {
    fn encode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}

/// This is fake function.
///
/// Fills the given `u8` array with a predictable sequence of bytes
fn fill_random(bytes: &mut [u8; 32]) {
    bytes.copy_from_slice(&[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ])
}

#[cfg(test)]
mod tests {
    use super::ClientRandom;

    #[test]
    fn client_random_generates_32_bytes_data() {
        let random_data = ClientRandom::new();
        assert_eq!(random_data.0.len(), 32);
    }
}
