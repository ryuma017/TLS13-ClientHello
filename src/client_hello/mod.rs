pub mod handshake;
pub mod tls_plaintext;

pub use handshake::HandshakeMessage;
pub use tls_plaintext::TLSPlaintext;

mod cipher_suites;
mod client_random;
mod client_version;
mod compression_methods;
mod enums;
mod extensions;
mod session_id;
mod utils;

use cipher_suites::CipherSuites;
use client_random::ClientRandom;
use client_version::ClientVersion;
use compression_methods::CompressionMethods;
use extensions::Extensions;
use session_id::SessionID;
use utils::{Encode, Legacy};

pub struct ClientHello {
    legacy_client_version: ClientVersion,
    client_random: ClientRandom,
    legacy_session_id: SessionID,
    cipher_suites: CipherSuites,
    legacy_compression_methods: CompressionMethods,
    extensions: Extensions,
}

impl ClientHello {
    pub fn default_v1_3() -> Self {
        Self {
            legacy_client_version: ClientVersion::legacy(),
            client_random: ClientRandom::new(),
            legacy_session_id: SessionID::legacy(),
            cipher_suites: CipherSuites::default(),
            legacy_compression_methods: CompressionMethods::legacy(),
            extensions: Extensions::default(),
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
        self.extensions.encode(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::{utils::Encode, ClientHello};

    #[test]
    fn client_hello_encoding_works() {
        let client_hello = ClientHello::default_v1_3();
        let encoded = client_hello.get_encoded_bytes();

        assert_eq!(
            encoded,
            vec![
                // --- Client Version ---
                0x03, 0x03, // TLS 1.2
                // --- Client Random ---
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
                0x1c, 0x1d, 0x1e, 0x1f,
                // --- Session ID ---
                0x20, // 0x20 (32) bytes of session ID follow
                0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed,
                0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb,
                0xfc, 0xfd, 0xfe, 0xff, // --- Cipher Suites ---
                0x00, 0x02, // 2 bytes of cipher suite data
                0x13, 0x01, // "TLS_AES_128_GCM_SHA256"
                // --- Compression Methods ---
                0x01, // 1 bytes of compression methods
                0x00, // "null" compression
                // --- Extensions length ---
                0x00, 0x57, // Extension - Supported Groups
                0x00, 0x0a, // assigned value for extension "supported groups"
                0x00, 0x16, // 0x16 (22) bytes of "supported group" extension data follows
                0x00, 0x14, // 0x14 (20) bytes of data are in the curves list
                0x00, 0x17, // "secp256r1"
                0x00, 0x18, // "secp384r1"
                0x00, 0x19, // "secp521r1"
                0x00, 0x1d, // "x25519"
                0x00, 0x1e, // "x448"
                0x01, 0x00, // "ffdhe2048"
                0x01, 0x01, // "ffdhe3072"
                0x01, 0x02, // "ffdhe4096"
                0x01, 0x03, // "ffdhe6144"
                0x01, 0x04, // "ffdhe8192"
                // Extension - Signature Algorithms
                0x00, 0x0d, // assigned value for extension "Signature Algorithms"
                0x00, 0x08, // 8 bytes of Signature Algorithms" extension data follows
                0x00, 0x06, // 6 bytes of data are in the following list of algorithms
                0x04, 0x03, // "ECDSA-SECP256r1-SHA256"
                0x05, 0x03, // "ECDSA-SECP384r1-SHA384"
                0x06, 0x03, // "ECDSA-SECP521r1-SHA512"
                // Extension - Supported Versions
                0x00, 0x2d, // assigned value for extension "Supported Versions"
                0x00, 0x03, // 3 bytes of "Supported Versions" extension data follows
                0x02, // 2 bytes of TLS versions follow
                0x03, 0x04, // "TLS 1.3"
                // Extension - Key Share
                0x00, 0x33, // assigned value for extension "Key Share"
                0x00, 0x26, // 0x26 (38) bytes of "Key Share" extension data follows
                0x00, 0x24, // 0x24 (36) bytes of key share data follows
                0x00, 0x1d, // "x25519" (key exchange via curve25519)
                0x00, 0x20, // 0x20 (32) bytes of public key follows
                0x21, 0x01, 0x9f, 0x6f, 0x71, 0x56, 0xe6, 0xab, 0x69, 0x2c, 0x6c, 0xed, 0x1a, 0xaf,
                0xa6, 0x31, 0x2f, 0x3e, 0xa5, 0xb9, 0xe0, 0x16, 0x64, 0x39, 0xcb, 0x73, 0x06, 0xbd,
                0x85, 0x64, 0x7e, 0x9a
            ]
        )
    }
}
