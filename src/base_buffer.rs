pub const MTU: usize = 1500;

#[repr(align(4))]
pub struct Buffer(pub [u8; MTU]);

impl Buffer {
    pub const fn new() -> Self {
        Self([0; MTU])
    }
}