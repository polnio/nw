use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct FlakeMetadata {
    pub locks: FlakeMetadataLocks,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocks {
    pub nodes: HashMap<String, FlakeMetadataLocksNodes>,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodes {
    pub original: Option<FlakeMetadataLocksNodesOriginal>,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesOriginal {
    pub r#ref: Option<String>,
}
