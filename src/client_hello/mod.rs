mod cipher_suites;
mod client_random;
mod client_version;
mod compression_methods;
mod extensions;
mod session_id;
mod utils;

use cipher_suites::CipherSuites;
use client_random::ClientRandom;
use client_version::ClientVersion;
use compression_methods::CompressionMethods;
use session_id::SessionID;
use utils::{Encode, Legacy};

struct ClientHello {
    legacy_client_version: ClientVersion,
    client_random: ClientRandom,
    legacy_session_id: SessionID,
    cipher_suites: CipherSuites,
    legacy_compression_methods: CompressionMethods,
    // extensions: Extensions,
}

impl ClientHello {
    fn new_v1_3() -> Self {
        Self {
            legacy_client_version: ClientVersion::legacy(),
            client_random: ClientRandom::new(),
            legacy_session_id: SessionID::legacy(),
            cipher_suites: CipherSuites::default(),
            legacy_compression_methods: CompressionMethods::legacy(),
            // TODO:
            // extensions: todo!(),
        }
    }
}

impl Encode for ClientHello {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.legacy_client_version.encode(bytes);
        self.client_random.encode(bytes);
        self.legacy_session_id.encode(bytes);
        self.cipher_suites.encode(bytes);
        self.legacy_compression_methods.encode(bytes);
        // self.extensions.encode(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::{utils::Encode, ClientHello};

    #[test]
    fn client_hello_encoding_works() {
        let client_hello = ClientHello::new_v1_3();
        let encoded = client_hello.get_encoded_bytes();

        assert_eq!(
            encoded,
            vec![
                0x03, 0x03, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
                0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
                0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6,
                0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4,
                0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff, 0x00, 0x02, 0x13,
                0x01, 0x01, 0x00
            ]
        )
    }
}
