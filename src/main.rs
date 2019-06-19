use atom::io::{IO, LMDB};
use atom::tree::node::Address;
use std::convert::TryInto;
use std::path::Path;

fn main() {
    let db = LMDB::new(Path::new("./bridge.atom2")).unwrap();
    let header = db.header().unwrap();
    println!("{}", serde_json::to_string(&header).unwrap());
    let a: Address = "@700002020167".parse().unwrap();
    db.block(&a);
}
