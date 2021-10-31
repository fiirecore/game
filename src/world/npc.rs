use hashbrown::HashMap;
use worldlib::character::npc::{NpcTypeId, NpcType};

pub type NpcTypes = HashMap<NpcTypeId, NpcType>;

#[deprecated]
pub static mut NPC_TYPES: Option<NpcTypes> = None;

pub fn npc_type(id: &NpcTypeId) -> &'static NpcType {
    unsafe{NPC_TYPES.as_ref()}.expect("Could not get Npc types!").get(id).unwrap_or_else(|| panic!("Could not get Npc type {}", id))
}