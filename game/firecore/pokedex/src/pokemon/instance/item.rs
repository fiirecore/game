use crate::{
    pokemon::{
        instance::PokemonInstance,
    },
    item::{
        script::{
            ItemScript,
            ItemActionKind,
        }
    }
};

impl PokemonInstance {

    pub fn execute_item_script(&mut self, script: &ItemScript) { // return result
        for action in &script.actions {
            match action {
                ItemActionKind::CurePokemon(status) => {
                    if let Some(effect) = self.data.status {
                        if let Some(status) = status {
                            if effect.status.eq(status) {
                                self.data.status = None;
                            }
                        } else {
                            self.data.status = None;
                        }
                    }
                }
                ItemActionKind::HealPokemon(hp) => {
                    self.current_hp += *hp;
                    if self.current_hp > self.base.hp {
                        self.current_hp = self.base.hp;
                    }
                }
            }
        }
	}

}