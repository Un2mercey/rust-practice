use crate::ImageDataErrors;
use std::convert::TryInto;

pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
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
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferToSmall);
        }
        self.data = data;
        Ok(())
    }
}
