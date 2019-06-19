#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Invalid attachment name: {}", name)]
    InvalidAttachmentName { name: String },
}
