use crate::engine::game_context::GameContext;
use crate::game::warp::warp_entry::WarpEntry;
use crate::util::file_util::asset_as_pathbuf;
use crate::entity::entity::Entity;
use super::world_manager::WorldConfig;
use super::world_manager::WorldManager;

impl WorldManager {

    pub fn warp_to_map(&mut self, warp: &mut WarpEntry) {
        self.world_map_manager.despawn();
        self.warp_map_manager.load_set(&warp.destination.world_id, 0);
        self.warp_map_manager.spawn();
    }

    pub(crate) fn world_warp(&mut self, context: &mut GameContext, entry: WarpEntry) {
        if entry.destination.world_id.as_str() != self.world_id.as_str() {

            self.player.coords.x = entry.destination.dest_x;
            self.player.coords.y = entry.destination.dest_y;

            let id = entry.destination.world_id;

            self.world_id = id.clone();

            let mut content = String::new();

            let mut filename = String::from("worlds/");
            filename.push_str(&id);
            filename.push_str("/");
            filename.push_str(&id);
            filename.push_str(".toml");
    
            match std::fs::read_to_string(asset_as_pathbuf(filename)) {
                Ok(string) => {
                    content = string;
                },
                Err(err) => {
                    println!("Error opening toml file {}", err);
                },
            }

            let toml: WorldConfig = toml::from_str(content.as_str()).unwrap();

            self.no_data_spawnpoint = toml.no_data_spawnpoint.unwrap();
            self.palette_sizes = toml.palette_sizes.unwrap();

            self.new_world_map(&id);
            self.load_maps(&id);
            self.load_world_map();
            self.populate_textures();

            
        } else if entry.destination.map_id.as_str().eq("world") {
            self.player.coords.x = entry.destination.dest_x;
            self.player.coords.y = entry.destination.dest_y;
            self.world_map_manager.reload(context);
        } else {
            self.world_map_manager.despawn();
            self.warp_map_manager.load_set(&entry.destination.map_id, entry.destination.map_set_num);
            self.warp_map_manager.spawn();
            self.warp_map_manager.reload();
            self.player.coords.x = entry.destination.dest_x;
            self.player.coords.y = entry.destination.dest_y;
        }
        
    }

    pub(crate) fn warp_warp(&mut self, context: &mut GameContext, entry: WarpEntry) {
        if entry.destination.map_id.as_str().eq("world") {
            self.warp_map_manager.despawn();
            self.world_map_manager.spawn();
            self.world_map_manager.get_current_world_mut().find_on_load(entry.destination.dest_x, entry.destination.dest_y);
            self.world_map_manager.reload(context);            
        } else {
            self.warp_map_manager.load_set(&entry.destination.map_id, entry.destination.map_set_num);
        }
        self.player.coords.x = entry.destination.dest_x;
        self.player.coords.y = entry.destination.dest_y;
    }

}