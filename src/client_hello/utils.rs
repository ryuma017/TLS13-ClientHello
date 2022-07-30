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

impl Encode for [u8] {
    fn encode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(self);
    }
}

pub trait Legacy: Sized {
    fn legacy() -> Self;
}

pub trait AssignedValue {
    type Integer: Encode;

    fn assigned_value(&self) -> Self::Integer;
}
