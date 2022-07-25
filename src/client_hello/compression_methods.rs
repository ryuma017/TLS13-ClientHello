pub struct CompressionMethods;

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::{Legacy, Encode};

    use super::CompressionMethods;

    #[test]
    fn compression_methods_encoding_works() {
        let compression_methods = CompressionMethods::legacy();
        let encoded = compression_methods.get_encoded_bytes();

        assert_eq!(encoded, vec![0x01, 0x00])
    }
}
