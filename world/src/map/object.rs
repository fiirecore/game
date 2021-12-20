use serde::{Deserialize, Serialize};

use firecore_utils::Coordinate;

#[derive(Deserialize, Serialize)]
pub struct MapObject {

    pub active: bool,
    pub id: usize,
    pub location: Coordinate,
    pub object_type: MapObjectType,

}

#[derive(Deserialize, Serialize)]
pub enum MapObjectType {

    Item,
    Tree,
    SmashableRock,
    PushableRock,

}