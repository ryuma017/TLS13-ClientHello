use std::ops::Deref;

use super::{
    enums::CompressionMethod,
    utils::{AssignedValue, Encode, Legacy},
};

/// TLS 1.3 no longer allows compression,
/// so this field is always a single entry with the "null" compression method
/// which performs no change to the data.
pub struct CompressionMethods(Vec<CompressionMethod>);

impl Encode for CompressionMethods {
    fn encode(&self, bytes: &mut Vec<u8>) {
        (self.len() as u8).encode(bytes);
        encode_compression_methods(bytes, self);
    }
}

impl Legacy for CompressionMethods {
    /// - `length` is set to `0x01` .
    /// - `values` is set to `CompressionMethod::Null` .
    fn legacy() -> Self {
        Self(vec![CompressionMethod::Null])
    }
}

fn encode_compression_methods(bytes: &mut Vec<u8>, values: &[CompressionMethod]) {
    values.iter().for_each(|cm| {
        cm.assigned_value().encode(bytes);
    })
}

impl Default for CompressionMethods {
    fn default() -> Self {
        Self(vec![CompressionMethod::Null])
    }
}

impl Deref for CompressionMethods {
    type Target = Vec<CompressionMethod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::client_hello::utils::{Encode, Legacy};

    use super::CompressionMethods;

    #[test]
    fn compression_methods_encoding_works() {
        let compression_methods = CompressionMethods::legacy();
        let encoded = compression_methods.get_encoded_bytes();

        assert_eq!(encoded, vec![0x01, 0x00])
    }
}
