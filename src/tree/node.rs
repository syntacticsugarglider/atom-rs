#[derive(Debug)]
pub struct Node {
    pub(crate) octant_mask: u8,
    pub(crate) branch_mask: u8,
}