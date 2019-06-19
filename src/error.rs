#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Invalid attachment name: {}", name)]
    InvalidAttachmentName { name: String },
    #[fail(display = "Invalid octant index: {}", index)]
    InvalidOctantIndex { index: u8 },
    #[fail(display = "Invalid node address: {}", address)]
    InvalidNodeAddress { address: String },
    #[fail(display = "Empty node address is invalid")]
    EmptyNodeAddress
}
