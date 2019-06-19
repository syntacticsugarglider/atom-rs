use atom::io::{IO, LMDB};
use atom::tree::node::Address;
use std::convert::TryInto;
use std::path::Path;

/*fn main() {
    let db = LMDB::new(Path::new("./bridge.atom2")).unwrap();
    let header = db.header().unwrap();
    println!("{}", serde_json::to_string(&header).unwrap());
}*/

fn main() {
    let a: Address = "@0202356".parse().unwrap();
    let b: Address = a.to_bytes().as_slice().try_into().unwrap();
    assert_eq!(a, b);
}
