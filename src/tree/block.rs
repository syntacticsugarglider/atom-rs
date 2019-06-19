pub struct Block<'a> {
    data: &'a [u32],
    capacity: u16,
    end_index: u16,
    is_dirty: bool,
    is_compact: bool,
}