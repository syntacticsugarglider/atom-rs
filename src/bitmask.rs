#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Bitmask {
    mask: u8
}

impl From<u8> for Bitmask {
    fn from(input: u8) -> Bitmask {
        Bitmask {
            mask: input
        }
    }
}

impl From<Bitmask> for u8 {
    fn from(input: Bitmask) -> u8 {
        input.mask
    }
}