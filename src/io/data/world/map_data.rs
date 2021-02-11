use serde::{Deserialize, Serialize};
use ahash::AHashSet as HashSet;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapData {

    pub battled: HashSet<String>,

}