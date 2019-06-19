pub mod types;

mod attachment_types;
pub use attachment_types::Type;

use std::marker::PhantomData;

pub type Schema = Vec<Box<dyn Type>>;

#[repr(transparent)]
pub struct Attachment<T: Type> {
    data: u32,
    ty: PhantomData<T>,
}

impl<T: Type> Attachment<T> {
    fn new<U: IntoAttachmentData>(data: U) -> Attachment<T> {
        Attachment {
            ty: PhantomData,
            data: data.into_attachment_data(),
        }
    }
}

pub trait FromAttachmentData {
    fn from_attachment_data_mut(data: &mut u32) -> &mut Self;
    fn from_attachment_data(data: &u32) -> &Self;
}

pub trait IntoAttachmentData {
    fn into_attachment_data(self) -> u32;
}

impl FromAttachmentData for u32 {
    fn from_attachment_data_mut(data: &mut u32) -> &mut u32 {
        data
    }
    fn from_attachment_data(data: &u32) -> &u32 {
        data
    }
}

impl IntoAttachmentData for u32 {
    fn into_attachment_data(self) -> u32 {
        self
    }
}

impl FromAttachmentData for [u8; 4] {
    fn from_attachment_data_mut(data: &mut u32) -> &mut [u8; 4] {
        unsafe { std::mem::transmute(data) }
    }
    fn from_attachment_data(data: &u32) -> &[u8; 4] {
        unsafe { std::mem::transmute(data) }
    }
}

impl IntoAttachmentData for [u8; 4] {
    fn into_attachment_data(self) -> u32 {
        unsafe { std::mem::transmute(self) }
    }
}

impl FromAttachmentData for [u16; 2] {
    fn from_attachment_data_mut(data: &mut u32) -> &mut [u16; 2] {
        unsafe { std::mem::transmute(data) }
    }
    fn from_attachment_data(data: &u32) -> &[u16; 2] {
        unsafe { std::mem::transmute(data) }
    }
}

impl IntoAttachmentData for [u16; 2] {
    fn into_attachment_data(self) -> u32 {
        unsafe { std::mem::transmute(self) }
    }
}

impl FromAttachmentData for f32 {
    fn from_attachment_data_mut(data: &mut u32) -> &mut f32 {
        unsafe { std::mem::transmute(data) }
    }
    fn from_attachment_data(data: &u32) -> &f32 {
        unsafe { std::mem::transmute(data) }
    }
}

impl IntoAttachmentData for f32 {
    fn into_attachment_data(self) -> u32 {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Type> Attachment<T> {
    fn get_mut<U: FromAttachmentData>(&mut self) -> &mut U {
        U::from_attachment_data_mut(&mut self.data)
    }
    fn get<U: FromAttachmentData>(&self) -> &U {
        U::from_attachment_data(&self.data)
    }
}

pub trait Averageable<T: Type> {
    fn average(self) -> Attachment<T>;
}

impl<T, U: Type> Averageable<U> for T
where
    T: IntoIterator<Item = Attachment<U>> + Sized,
{
    fn average(self) -> Attachment<U> {
        U::averager(self)
    }
}
