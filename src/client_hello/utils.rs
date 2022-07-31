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

#[allow(non_camel_case_types)]
pub struct u24(pub u32);

impl Encode for u24 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let be32 = u32::to_be_bytes(self.0);
        bytes.extend_from_slice(&be32[1..]);
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
    type UInt: Encode;

    fn assigned_value(&self) -> Self::UInt;
}
