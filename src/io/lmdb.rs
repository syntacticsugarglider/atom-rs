use lmdb::{Environment, Database, EnvironmentFlags, Transaction};
use std::{path::Path, ffi::CString};
use failure::Error;
use crate::{tree::Header, io::IO};

pub struct LMDB {
    environment: Environment,
    database: Database,
}

impl LMDB {
    pub fn new(path: &'_ Path) -> Result<LMDB, Error> {
        let environment = Environment::new().set_flags(EnvironmentFlags::NO_SUB_DIR).open(path)?;
        let database = environment.open_db(None)?;
        let db = LMDB {
            environment,
            database,
        };
        Ok(db)
    }
}

impl IO for LMDB {
    fn header(&self) -> Result<Header, Error> {
        let transaction = self.environment.begin_ro_txn()?;
        let header_data = CString::new(transaction.get(self.database, &"!header".to_owned())?)?.to_str()?.to_owned();
        let header: Header = serde_json::from_str(&header_data).unwrap();
        Ok(header)
    }
}