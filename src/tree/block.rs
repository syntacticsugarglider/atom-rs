use crate::tree::node::Node;

pub struct Block {
    parent_node: Node,
    attachment_count: u8,
    data: *const u32,
    capacity: u16,
    end_index: u16,
    is_dirty: bool,
    is_compact: bool,
}
