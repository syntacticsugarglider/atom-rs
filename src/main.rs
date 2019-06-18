use lmdb::{Environment, EnvironmentFlags, Transaction, Database};
use serde::{Deserialize, Serialize, de::{Deserializer, self}};
use std::{marker::PhantomData, fmt::{Debug, Formatter, Display, self}, ffi::CString, path::Path, collections::HashMap, str::FromStr, iter::IntoIterator};
#[macro_use] extern crate failure;
use failure::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HeaderData {
    schema: Vec<String>,
    #[serde(rename = "rootNode")]
    root_node: (u8, u8),
    #[serde(default)]
    compressed: bool,
    #[serde(default)]
    building: bool,
    #[serde(default)]
    metadata: HashMap<String, String>

}

#[derive(Fail, Debug)]
enum AtomError {
    #[fail(display = "Invalid attachment name: {}", name)]
    InvalidAttachmentName {
        name: String
    }
}

#[repr(transparent)]
struct Attachment<T: AttachmentType> {
    data: u32,
    ty: PhantomData<T>,
}

impl<T: AttachmentType> Attachment<T> {
    fn new(data: u32) -> Attachment<T> {
        Attachment {
            ty: PhantomData,
            data
        }
    }
}

trait FromAttachmentData {
    fn from_attachment_data_mut(data: &mut u32) -> &mut Self;
    fn from_attachment_data(data: &u32) -> &Self;
}

impl FromAttachmentData for [u8; 4] {
    fn from_attachment_data_mut(data: &mut u32) -> &mut [u8; 4] {
        unsafe { std::mem::transmute(data) }
    }
    fn from_attachment_data(data: &u32) -> &[u8; 4] {
        unsafe { std::mem::transmute(data) }
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
impl FromAttachmentData for f32 {
    fn from_attachment_data_mut(data: &mut u32) -> &mut f32 {
        unsafe { std::mem::transmute(data) }
    }
    fn from_attachment_data(data: &u32) -> &f32 {
        unsafe { std::mem::transmute(data) }
    }
}

impl<T: AttachmentType> Attachment<T> {
    fn get_mut<U: FromAttachmentData>(&mut self) -> &mut U {
        U::from_attachment_data_mut(&mut self.data)
    }
    fn get<U: FromAttachmentData>(&self) -> &U {
        U::from_attachment_data(&self.data)
    }
}

struct RGBA;

impl AttachmentType for RGBA {
    fn name(&self) -> String {
        "RGBA".to_owned()
    }
    fn averager<T>(attachments: T) -> Attachment<Self> where T: IntoIterator<Item = Attachment<Self>> {
        let summed = attachments.into_iter().fold((0usize, [0u16; 4]), |current, attachment| {
            let mut current = current;
            current.1.iter_mut().enumerate().for_each(|(index, item)| {
                *item += attachment.get::<[u8; 4]>()[index] as u16
            });
            (current.0 + 1, current.1)
        });
        let mut data = [0u8; 4];
        summed.1.iter().zip(data.iter_mut()).for_each(|(d, s)| {
            *s = (d / summed.0 as u16) as u8;
        });
        Attachment::new(unsafe {
            std::mem::transmute(data)
        })
    }
}

trait AttachmentType {
    fn averager<T>(attachments: T) -> Attachment<Self> where T: IntoIterator<Item = Attachment<Self>>, Self: Sized;
    fn name(&self) -> String;
}

impl Debug for AttachmentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AttachmentType ( {} )", self.name())
    }
}

impl FromStr for Box<dyn AttachmentType> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RGBA" => Ok(Box::new(RGBA)),
            _ => Err(AtomError::InvalidAttachmentName {
                name: s.to_owned()
            })?
        }
    }
}

type AttachmentSchema = Vec<Box<dyn AttachmentType>>;

#[derive(Debug)]
struct Header {
    schema: AttachmentSchema,
    root_node: Node,
    metadata: HashMap<String, String>,
    building: bool,
    compressed: bool,
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data: HeaderData = HeaderData::deserialize(deserializer)?;
        let schema: Result<Vec<Box<dyn AttachmentType>>, Error> = data.schema.iter().map(|name| name.parse()).collect();
        Ok(Header {
            metadata: data.metadata,
            building: data.building,
            compressed: data.compressed,
            schema: schema.map_err(|err| de::Error::custom(err))?,
            root_node: Node {
                octant_mask: data.root_node.0,
                branch_mask: data.root_node.1,
            }
        })
    }
}

trait IO {
    fn header(&self) -> Result<Header, Error>;
}

struct LMDB {
    environment: Environment,
    database: Database,
}

impl LMDB {
    fn new(path: &'_ Path) -> Result<LMDB, Error> {
        let environment = Environment::new().set_flags(EnvironmentFlags::NO_SUB_DIR).open(path)?;
        let database = environment.open_db(None)?;
        let db = LMDB {
            environment,
            database,
        };
        Ok(db)
    }
}

impl IO for LMDB {
    fn header(&self) -> Result<Header, Error> {
        let transaction = self.environment.begin_ro_txn()?;
        let header_data = CString::new(transaction.get(self.database, &"!header".to_owned())?)?.to_str()?.to_owned();
        let header: Header = serde_json::from_str(&header_data).unwrap();
        Ok(header)
    }
}

#[derive(Debug)]
struct Node {
    octant_mask: u8,
    branch_mask: u8,
}

fn main() {
    let db = LMDB::new(Path::new("./bridge.atom2")).unwrap();
    let header = db.header();
    println!("{:?}", header);
}