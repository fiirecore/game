use std::collections::HashMap;

use crate::game::world::warp_map::warp_map::WarpMap;

pub struct WarpMapSet {

    pub maps: HashMap<usize, WarpMap>,
    pub current_map_index: usize, 

}
impl WarpMapSet {

    pub fn new() -> WarpMapSet {

        WarpMapSet {

            maps: HashMap::new(),
            current_map_index: 0,

        }

    }

    pub fn current_map(&self) -> &WarpMap {
        return self.maps.get(&self.current_map_index).expect("Could not get current warp map!");
    }

    pub fn current_map_mut(&mut self) -> &mut WarpMap {
        return self.maps.get_mut(&self.current_map_index).expect("Could not get current warp map!");
    }

    pub fn walkable(&self, x: isize, y: isize) -> u8 {
        let map = self.current_map();
        let index = (x + y * map.width as isize) as usize;
        if index >= map.movement_map.len() {
            return 1;
        } else {
            for npc in &map.npcs {
                if x == npc.x {
                    if y == npc.y {
                        return 1;
                    }
                }
            }
            return map.movement_map[index];
        }
    }

}