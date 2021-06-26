use crate::deps::{
    str::TinyStr16,
    hash::HashMap,
};

use worldlib::character::npc::npc_type::NpcType;


pub type NpcTypes = HashMap<TinyStr16, NpcType>;

pub static mut NPC_TYPES: Option<NpcTypes> = None;

pub fn npc_type(id: &TinyStr16) -> &'static NpcType {
    unsafe{NPC_TYPES.as_ref()}.expect("Could not get Npc types!").get(id).unwrap_or_else(|| panic!("Could not get Npc type {}", id))
}