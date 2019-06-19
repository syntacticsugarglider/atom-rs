use crate::{
    io::IO,
    tree::{node::Address, Header, Block},
};
use failure::Error;
use lmdb::{Cursor, Database, Environment, EnvironmentFlags, Transaction};
use std::{convert::TryInto, ffi::CString, path::Path};

pub struct LMDB {
    environment: Environment,
    database: Database,
}

impl LMDB {
    pub fn new(path: &'_ Path) -> Result<LMDB, Error> {
        let environment = Environment::new()
            .set_flags(EnvironmentFlags::NO_SUB_DIR)
            .open(path)?;
        let database = environment.open_db(None)?;
        let db = LMDB {
            environment,
            database,
        };
        Ok(db)
    }
    pub fn block(&self, address: &Address) -> Result<Block, Error> {
        let transaction = self.environment.begin_ro_txn().unwrap();
        let data = 
            transaction
                .get(self.database, &address.to_string())?;
        Ok(Block {})
    }
}

impl IO for LMDB {
    fn header(&self) -> Result<Header, Error> {
        let transaction = self.environment.begin_ro_txn()?;
        let header_data = CString::new(transaction.get(self.database, &"!header".to_owned())?)?
            .to_str()?
            .to_owned();
        let header: Header = serde_json::from_str(&header_data).unwrap();
        Ok(header)
    }
}
