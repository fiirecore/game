use crate::deps::{
    str::TinyStr16,
    hash::HashMap,
};

use worldlib::character::npc::npc_type::NPCType;


pub type NPCTypes = HashMap<TinyStr16, NPCType>;

pub static mut NPC_TYPES: Option<NPCTypes> = None;

pub fn npc_type(id: &TinyStr16) -> &'static NPCType {
    unsafe{NPC_TYPES.as_ref()}.expect("Could not get NPC types!").get(id).unwrap_or_else(|| panic!("Could not get NPC type {}", id))
}