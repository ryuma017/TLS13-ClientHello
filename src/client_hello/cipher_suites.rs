use super::Encode;

pub struct CipherSuites;

#[cfg(test)]
mod tests {
    use crate::client_hello::Encode;

    use super::CipherSuites;

    #[test]
    fn default_cipher_suites_encoding_works() {
        let cipher_suites = CipherSuites::default();
        let encoded = cipher_suites.get_encoded_bytes();

        assert_eq!(encoded, [
            0x00, 0x02, 0x13, 0x01
        ]);
    }
}
