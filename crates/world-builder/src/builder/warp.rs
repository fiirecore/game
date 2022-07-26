use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use world::map::warp::{WarpDestination, WarpEntry, Warps};

use crate::builder::structs::BuilderArea;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BuilderWarpEntry {
    pub area: BuilderArea,
    pub destination: WarpDestination,
}

pub fn load_warp_entries(warp_path: PathBuf) -> Warps {
    read_dir(warp_path)
        .map(|dir| {
            dir.flatten()
                .map(|dir| dir.path())
                .map(|path| {
                    (
                        read_to_string(&path).unwrap_or_else(|err| {
                            panic!("Could not get warp file at {:?} with error {}", path, err)
                        }),
                        path,
                    )
                })
                .map(|(data, path)| {
                    /*(filename.to_string_lossy().split('.').next().unwrap().parse().unwrap_or_else(|err| panic!("Warp file name \"{:?}\" could not be parsed into ASCII with error {}", filename, err)), */
                    ron::from_str::<BuilderWarpEntry>(&data).unwrap_or_else(|err| {
                        panic!(
                            "Could not parse warp entry at {:?} with error {}",
                            path, err
                        )
                    }).into()
                })
                .collect()
        })
        .unwrap_or_default()
}

impl From<BuilderWarpEntry> for WarpEntry {
    fn from(entry: BuilderWarpEntry) -> Self {
        Self {
            area: entry.area.into(),
            destination: entry.destination,
        }
    }
}
