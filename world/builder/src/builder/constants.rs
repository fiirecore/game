use std::path::Path;

use worldlib::map::manager::constants::WorldMapManagerConstants;

pub fn get_constants(root_path: &Path) -> WorldMapManagerConstants {
    ron::from_str(&std::fs::read_to_string(root_path.join("center.ron")).unwrap()).unwrap()
}