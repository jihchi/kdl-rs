use std::{collections::HashMap, str::FromStr};

use crate::{KdlNode, KdlSchemaError};

pub struct KdlSchema(KdlNode);

impl KdlSchema {
    pub fn from_node(node: KdlNode) -> Result<KdlSchema, KdlSchemaError> {
        Ok(KdlSchema(node))
    }
}

impl FromStr for KdlSchema {
    type Err = KdlSchemaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = crate::parse_document(s)?;
        KdlSchema::from_node(nodes.pop().unwrap_or_else(|| KdlNode {
            name: "".to_string(),
            values: vec![],
            children: vec![],
            properties: HashMap::new(),
        }))
    }
}
