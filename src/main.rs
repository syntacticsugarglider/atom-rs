use atom::io::{IO, LMDB};
use std::path::Path;

fn main() {
    let db = LMDB::new(Path::new("./bridge.atom2")).unwrap();
    let header = db.header().unwrap();
    println!("{}", serde_json::to_string(&header).unwrap());
}
