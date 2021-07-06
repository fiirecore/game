use std::{cell::UnsafeCell, rc::Rc};

use battlelib::{
    client::{BattleClient, BattleEndpoint},
    message::{ClientMessage, ServerMessage},
};
use crate::{
    deps::borrow::BorrowableMut,
    engine::tetra::Context,
    pokedex::{
        item::bag::Bag,
        gui::{bag::BagGui, party::PartyGui},
    },
};

use firecore_battle_gui::BattlePlayerGui;

#[derive(Clone)]
pub struct BattlePlayerGuiRef<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord>(
    pub Rc<UnsafeCell<BattlePlayerGui<ID>>>,
);

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattlePlayerGuiRef<ID> {
    pub fn new(
        ctx: &mut Context,
        party: Rc<PartyGui>,
        bag: Rc<BagGui>,
        player_bag: BorrowableMut<'static, Bag>,
        id_default: ID,
    ) -> Self {
        Self(Rc::new(std::cell::UnsafeCell::new(BattlePlayerGui::new(
            ctx, party, bag, player_bag, id_default,
        ))))
    }
    pub fn get(&self) -> &mut BattlePlayerGui<ID> {
        unsafe { &mut *self.0.get() }
    }
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattleEndpoint<ID>
    for BattlePlayerGuiRef<ID>
{
    fn give_client(&mut self, message: ServerMessage<ID>) {
        self.get().give_client(message)
    }
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattleClient<ID>
    for BattlePlayerGuiRef<ID>
{
    fn give_server(&mut self) -> Option<ClientMessage> {
        self.get().give_server()
    }
}
