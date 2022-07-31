use super::{
    enums::{ContentType, ProtocolVersion},
    utils::{AssignedValue, Encode},
};

pub struct TLSPlaintext {
    content_type: ContentType,
    protocol_version: ProtocolVersion,
    payload: Vec<u8>,
}

impl TLSPlaintext {
    fn new_handshake(payload: Vec<u8>) -> Self {
        let content_type = ContentType::Handshake;
        let protocol_version = ProtocolVersion::TLSv1_0;

        Self {
            content_type,
            protocol_version,
            payload,
        }
    }
}

impl Encode for TLSPlaintext {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.content_type.assigned_value().encode(bytes);
        self.protocol_version.assigned_value().encode(bytes);
        (self.payload.len() as u16).encode(bytes);
        bytes.extend_from_slice(&self.payload);
    }
}

#[cfg(test)]
mod tests {
    use crate::client_hello::{handshake::HandshakeMessage, utils::Encode, ClientHello};

    use super::TLSPlaintext;

    #[test]
    fn handshake_tls_plaintext_encoding_works() {
        let client_hello = ClientHello::new_v1_3();
        let client_hello_handshake = HandshakeMessage::new_client_hello(client_hello);
        let payload = client_hello_handshake.get_encoded_bytes();
        let tls_plaintext = TLSPlaintext::new_handshake(payload);
        let encoded = tls_plaintext.get_encoded_bytes();

        assert_eq!(
            encoded[..5],
            vec![
                0x16, // type is 0x16 (handshake record)
                0x03, 0x01, // protocol version is "3,1" (TLS 1.0)
                0x00, 0xa6, // 0xa6 (166) bytes of handshake message follows
            ]
        )
    }
}
