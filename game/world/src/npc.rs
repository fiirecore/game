use game::deps::{
    tinystr::TinyStr16,
    hash::HashMap,
};

use firecore_world_lib::character::npc::npc_type::NPCType;


pub type NPCTypes = HashMap<TinyStr16, NPCType>;

pub static mut NPC_TYPES: Option<NPCTypes> = None;

pub fn npc_type(id: &TinyStr16) -> Option<&NPCType> {
    unsafe{NPC_TYPES.as_ref()}.expect("Could not get NPC types!").get(id)
}