pub struct MapWindowManager {

    alive: bool,

}

impl MapWindowManager {

    pub fn new() -> MapWindowManager {
        MapWindowManager {
            alive: false,
        }
    }

}

impl Default for MapWindowManager {
    fn default() -> Self {
        Self::new()
    }
}