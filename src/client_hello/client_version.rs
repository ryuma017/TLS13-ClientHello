use super::{Encode, Legacy};

/// TLS 1.3 session must be disguised as a TLS 1.2 session.
/// This field is no longer used for version negotiation
/// and is hardcoded to the 1.2 version.
pub struct ClientVersion(u16);

impl Encode for ClientVersion {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.0.encode(bytes)
    }
}

impl Legacy for ClientVersion {
    /// Always `0x0303` (TLS 1.2) must be set
    fn legacy() -> Self {
        Self(0x0303)
    }
}
