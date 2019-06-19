use std::marker::PhantomData;

struct Pool<'a, T> {
    data: &'a mut [u32],
    entry_count: u16,
    free_count: u16,
    data_type: PhantomData<T>,
}

impl<'a, T> Pool<'a, T> {
    fn new() -> Pool<'a, T> {
        Pool {
            data: &mut [],
            entry_count: 0,
            free_count: 0,
            data_type: PhantomData,
        }
    }
}