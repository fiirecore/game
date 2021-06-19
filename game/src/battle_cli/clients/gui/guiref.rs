use std::{cell::UnsafeCell, rc::Rc};

use deps::tetra::Context;
use firecore_battle::{
    client::{BattleClient, BattleEndpoint},
    message::{ClientMessage, ServerMessage},
};

use crate::gui::{bag::BagGui, party::PartyGui};

use super::BattlePlayerGui;

#[derive(Clone)]
pub struct BattlePlayerGuiRef(pub Rc<UnsafeCell<BattlePlayerGui>>);

impl BattlePlayerGuiRef {
    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self(Rc::new(std::cell::UnsafeCell::new(BattlePlayerGui::new(
            ctx, party, bag,
        ))))
    }
    pub fn get(&self) -> &mut BattlePlayerGui {
        unsafe { self.0.get().as_mut().unwrap() }
    }
}

impl BattleEndpoint for BattlePlayerGuiRef {
    fn give_client(&mut self, message: ServerMessage) {
        self.get().give_client(message)
    }
}

impl BattleClient for BattlePlayerGuiRef {
    fn give_server(&mut self) -> Option<ClientMessage> {
        self.get().give_server()
    }
}
