use std::convert::TryInto;

pub struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity: u32 = height * width * 4;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }
}
