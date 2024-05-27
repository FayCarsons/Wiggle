use chrono::NaiveDateTime;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData {
    local_diff: HashMap<String, NaiveDateTime>,
}

impl MetaData {
    pub fn from_file() -> std::io::Result<Self> {
        todo!()
    }
}
