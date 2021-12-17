use std::rc::Rc;

use crate::{
    saves::{GameBag, GamePokemon},
    split, Receiver, Sender,
};
use firecore_battle::pokedex::pokemon::party::Party;
use firecore_battle_gui::pokedex::{
    context::PokedexClientData,
    engine::{
        error::ImageError,
        input::controls::{pressed, Control},
        util::Entity,
        Context,
    },
    gui::{bag::BagGui, party::PartyGui},
};
use worldlib::{character::player::PlayerCharacter, serialized::SerializedWorld};

use crate::{saves::PlayerData, state::game::GameActions};

use super::{
    gui::{StartMenu, TextWindow},
    map::manager::GameWorldMapManager,
    WorldActions,
};

mod command;

pub struct WorldManager {
    map: GameWorldMapManager,

    menu: StartMenu,
    text: TextWindow,

    sender: Sender<GameActions>,
    receiver: Receiver<WorldActions>,
}

impl WorldManager {
    pub(crate) fn new(
        ctx: &mut Context,
        dex: Rc<PokedexClientData>,
        party: Rc<PartyGui>,
        bag: Rc<BagGui>,
        sender: Sender<GameActions>,
    ) -> Result<Self, ImageError> {
        let (world, receiver) = split();
        Ok(Self {
            map: GameWorldMapManager::new(ctx, world),
            menu: StartMenu::new(dex, party, bag, sender.clone()),
            text: TextWindow::new(ctx)?,
            sender,
            receiver,
        })
    }

    pub fn load(&mut self, ctx: &mut Context, world: SerializedWorld) {
        self.map.load(ctx, world)
    }

    pub fn seed(&mut self, seed: u64) {
        self.map.randoms.seed(seed)
    }

    pub fn start(
        &mut self,
        ctx: &mut Context,
        player: &mut PlayerCharacter,
        party: &mut Party<GamePokemon>,
    ) {
        self.map.on_start(ctx, player, party)
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        delta: f32,
        player: &mut PlayerCharacter,
        party: &mut Party<GamePokemon>,
        bag: &GameBag,
    ) {
        if self.menu.alive() {
            self.menu.update(ctx, delta, party, bag);
        } else {
            if pressed(ctx, Control::Start) {
                self.menu.spawn();
            }

            #[deprecated]
            if pressed(ctx, Control::B) {
                player.unfreeze();
            }

            self.map.update(ctx, player, party, &mut self.text, delta);
        }
    }

    pub fn draw(&self, ctx: &mut Context, save: &PlayerData) {
        if !self.menu.fullscreen() {
            self.map.draw(ctx, &save.character);
            self.text.draw(ctx);
        }
        self.menu.draw(ctx, save)
    }
}
