pub struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

pub impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        FloatingImage {
            width,
            height,
            data: vec![],
            name,
        }
    }
}
