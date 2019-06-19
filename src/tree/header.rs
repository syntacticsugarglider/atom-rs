use crate::{
    attachment::{Schema, Type},
    tree::Node,
};

use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize, Serializer,
};

use failure::Error;

use std::collections::HashMap;

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
    metadata: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Header {
    schema: Schema,
    root_node: Node,
    metadata: HashMap<String, String>,
    building: bool,
    compressed: bool,
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: HeaderData = HeaderData::deserialize(deserializer)?;
        let schema: Result<Vec<Box<dyn Type>>, Error> =
            data.schema.iter().map(|name| name.parse()).collect();
        Ok(Header {
            metadata: data.metadata,
            building: data.building,
            compressed: data.compressed,
            schema: schema.map_err(de::Error::custom)?,
            root_node: Node::root(
                data.root_node.0.into(),
                data.root_node.1.into(),
            )
        })
    }
}

impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        HeaderData::serialize(
            &HeaderData {
                building: self.building,
                compressed: self.compressed,
                metadata: self.metadata.clone(),
                root_node: (self.root_node.octant_mask.into(), self.root_node.branch_mask.into()),
                schema: self.schema.iter().map(|ty| ty.name()).collect(),
            },
            serializer,
        )
    }
}
