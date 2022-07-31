use super::{
    enums::{ContentType, ProtocolVersion},
    utils::{AssignedValue, Encode},
};

struct Payload(Vec<u8>);

pub struct TLSPlaintext {
    content_type: ContentType,
    protocol_version: ProtocolVersion,
    payload: Payload,
}

impl TLSPlaintext {
    pub fn new_handshake(payload: impl Encode) -> Self {
        let content_type = ContentType::Handshake;
        let protocol_version = ProtocolVersion::TLSv1_0;
        let payload = Payload(payload.get_encoded_bytes());

        Self {
            content_type,
            protocol_version,
            payload,
        }
    }

    pub fn encode(&self, bytes: &mut Vec<u8>) {
        self.content_type.assigned_value().encode(bytes);
        self.protocol_version.assigned_value().encode(bytes);
        (self.payload.0.len() as u16).encode(bytes);
        bytes.extend_from_slice(&self.payload.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::client_hello::{ClientHello, HandshakeMessage};

    use super::TLSPlaintext;

    #[test]
    fn handshake_tls_plaintext_encoding_works() {
        let client_hello = ClientHello::default_v1_3();
        let client_hello_handshake = HandshakeMessage::new_client_hello(client_hello);
        let tls_plaintext = TLSPlaintext::new_handshake(client_hello_handshake);

        let mut bytes = Vec::new();
        tls_plaintext.encode(&mut bytes);

        assert_eq!(
            bytes[..5],
            vec![
                0x16, // type is 0x16 (handshake record)
                0x03, 0x01, // protocol version is "3,1" (TLS 1.0)
                0x00, 0xa6, // 0xa6 (166) bytes of handshake message follows
            ]
        )
    }
}
