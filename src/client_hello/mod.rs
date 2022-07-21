mod cipher_suites;
mod client_random;
mod client_version;
mod compression_methods;
mod extensions;
mod session_id;

pub use cipher_suites::*;
pub use client_random::*;
pub use client_version::*;
pub use compression_methods::*;
pub use extensions::*;
pub use session_id::*;

struct ClientHello {
    legacy_client_version: ClientVersion,
    client_random: ClientRandom,
    legacy_session_id: SessionID,
    cipher_suites: CipherSuites,
    legacy_compression_methods: CompressionMethods,
    extensions: Extensions,
}

impl ClientHello {
    fn new_v1_3() -> Self {
        Self {
            legacy_client_version: ClientVersion::legacy(),
            client_random: todo!(),
            legacy_session_id: todo!(),
            cipher_suites: todo!(),
            legacy_compression_methods: todo!(),
            extensions: todo!(),
        }
    }
}

impl Encode for ClientHello {
    fn encode(&self, _bytes: &mut Vec<u8>) {
        todo!()
    }
}

pub trait Encode {
    fn encode(&self, bytes: &mut Vec<u8>);

    fn get_encoded_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        self.encode(&mut b);
        b
    }
}

impl Encode for u8 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        bytes.push(*self);
    }
}

impl Encode for u16 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let be16: [u8; 2] = u16::to_be_bytes(*self);
        bytes.extend_from_slice(&be16);
    }
}

pub trait Legacy: Sized {
    fn legacy() -> Self;
}
