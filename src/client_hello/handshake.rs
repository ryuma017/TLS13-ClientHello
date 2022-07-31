use super::{
    enums::{HandshakeData, HandshakeType},
    utils::{u24, AssignedValue, Encode},
    ClientHello,
};

pub struct HandshakeMessage {
    hs_type: HandshakeType,
    data: HandshakeData,
}

impl HandshakeMessage {
    pub fn new_client_hello(data: ClientHello) -> Self {
        Self {
            hs_type: HandshakeType::ClientHello,
            data: HandshakeData::ClientHello(data),
        }
    }
}

impl Encode for HandshakeMessage {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.hs_type.assigned_value().encode(bytes);
        let mut sub = Vec::new();
        self.data.encode(&mut sub);
        u24(sub.len() as u32).encode(bytes);
        bytes.extend_from_slice(&sub);
    }
}

#[cfg(test)]
mod tests {
    use crate::client_hello::{utils::Encode, ClientHello};

    use super::HandshakeMessage;

    #[test]
    fn client_hello_handshake_message_encoding_works() {
        let ch_data = ClientHello::default_v1_3();
        let handshake_message = HandshakeMessage::new_client_hello(ch_data);
        let encoded = handshake_message.get_encoded_bytes();

        assert_eq!(
            encoded[..4],
            vec![
                0x01, // handshake message type 0x01 (client hello)
                0x00, 0x00, 0xa2, // 0xa2 (162) bytes of client hello data follows
            ]
        );
    }
}
