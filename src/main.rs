use std::path::Path;
use atom::io::{LMDB, IO};

fn main() {
    let db = LMDB::new(Path::new("./bridge.atom2")).unwrap();
    let header = db.header().unwrap();
    println!("{}", serde_json::to_string(&header).unwrap());
}