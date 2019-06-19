use crate::attachment::{Type, Attachment};

pub struct Normal;

impl Type for Normal {
    fn name(&self) -> String {
        "Normal".to_owned()
    }
    fn averager<T>(attachments: T) -> Attachment<Self> where T: IntoIterator<Item = Attachment<Self>> {
        Attachment::new(0)
    }
}