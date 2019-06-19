use crate::tree::{Block, Header};

use failure::Error;

mod lmdb;
pub use self::lmdb::LMDB;

pub trait IO {
    fn header(&self) -> Result<Header, Error>;
    //fn block(&self) -> Result<Block, Error>;
}
