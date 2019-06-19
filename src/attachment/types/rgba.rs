use crate::attachment::{Attachment, Type};

pub struct RGBA;

impl Type for RGBA {
    fn name(&self) -> String {
        "RGBA".to_owned()
    }
    fn averager<T>(attachments: T) -> Attachment<Self>
    where
        T: IntoIterator<Item = Attachment<Self>>,
    {
        let summed = attachments
            .into_iter()
            .fold((0usize, [0u16; 4]), |current, attachment| {
                let mut current = current;
                current
                    .1
                    .iter_mut()
                    .zip(attachment.get::<[u8; 4]>().iter())
                    .for_each(|(item, attachment_item)| *item += *attachment_item as u16);
                (current.0 + 1, current.1)
            });
        let mut data = [0u8; 4];
        summed.1.iter().zip(data.iter_mut()).for_each(|(d, s)| {
            *s = (d / summed.0 as u16) as u8;
        });
        Attachment::new(data)
    }
}
