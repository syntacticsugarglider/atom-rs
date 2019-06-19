use crate::{
    attachment::{
        types::{Normal, RGBA},
        Attachment,
    },
    error,
};
use failure::Error;
use std::{
    fmt::{self, Debug, Formatter},
    str::FromStr,
};

pub trait Type {
    fn averager<T>(attachments: T) -> Attachment<Self>
    where
        T: IntoIterator<Item = Attachment<Self>>,
        Self: Sized;
    fn name(&self) -> String;
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AttachmentType ( {} )", self.name())
    }
}

impl FromStr for Box<dyn Type> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RGBA" => Ok(Box::new(RGBA)),
            "Normal" => Ok(Box::new(Normal)),
            _ => Err(error::Error::InvalidAttachmentName { name: s.to_owned() })?,
        }
    }
}
