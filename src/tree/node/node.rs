use crate::bitmask::Bitmask;

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct NodePointer {
    pointer: u16,
}

impl NodePointer {
    pub const FAR: NodePointer = NodePointer { pointer: 0xffff };
}

#[derive(Debug)]
#[repr(C)]
pub struct Node {
    octants_index: NodePointer,
    pub(crate) octant_mask: Bitmask,
    pub(crate) branch_mask: Bitmask,
}

impl Node {
    pub(crate) fn root(octant_mask: Bitmask, branch_mask: Bitmask) -> Node {
        Node {
            octants_index: NodePointer::FAR,
            octant_mask,
            branch_mask,
        }
    }
}
