use game::{
    input::{pressed, Control},
    pokedex::{
        battle::party::knowable::BattlePartyUnknown, item::ItemRef, moves::target::MoveTarget,
        pokemon::instance::PokemonInstance,
    },
    tetra::Context,
    util::{Entity, Reset},
};

use self::{battle::BattleOptions, fight::FightPanel, target::TargetPanel};

pub mod move_info;
pub mod moves;
pub mod target;

pub mod battle;
pub mod fight;

pub mod level;

pub struct BattlePanel {
    alive: bool,

    pub active: BattlePanels,

    pub battle: BattleOptions,
    pub fight: FightPanel,
    pub targets: TargetPanel,
}

pub enum BattlePanels {
    Main,
    Fight,
    Target(MoveTarget, Option<ItemRef>),
}

impl Default for BattlePanels {
    fn default() -> Self {
        Self::Main
    }
}

impl BattlePanel {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            alive: false,
            active: BattlePanels::default(),
            battle: BattleOptions::new(ctx),
            fight: FightPanel::new(ctx),
            targets: TargetPanel::new(ctx),
        }
    }

    pub fn user(&mut self, instance: &PokemonInstance) {
        self.battle.setup(instance);
        self.fight.user(instance);
        self.battle.cursor = 0;
        self.fight.moves.cursor = 0;
        self.targets.cursor = 0;
        self.spawn();
    }

    pub fn target<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord>(
        &mut self,
        targets: &BattlePartyUnknown<ID>,
    ) {
        self.targets.update_names(targets);
    }

    pub fn input(&mut self, ctx: &Context, pokemon: &PokemonInstance) -> Option<BattlePanels> {
        if self.alive {
            match self.active {
                BattlePanels::Main => {
                    self.battle.input(ctx);
                    pressed(ctx, Control::A).then(|| BattlePanels::Main)
                }
                BattlePanels::Fight => {
                    if pressed(ctx, Control::B) {
                        self.active = BattlePanels::Main;
                    }
                    self.fight.input(ctx, pokemon);
                    pressed(ctx, Control::A).then(|| BattlePanels::Fight)
                }
                BattlePanels::Target(..) => {
                    if pressed(ctx, Control::B) {
                        self.active = BattlePanels::Fight;
                    }
                    self.targets.input(ctx);
                    pressed(ctx, Control::A).then(|| std::mem::take(&mut self.active))
                }
            }
        } else {
            None
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            match self.active {
                BattlePanels::Main => self.battle.draw(ctx),
                BattlePanels::Fight => self.fight.draw(ctx),
                BattlePanels::Target(..) => self.targets.draw(ctx),
            }
        }
    }
}

impl Entity for BattlePanel {
    fn spawn(&mut self) {
        self.alive = true;
        self.active = BattlePanels::default();
        self.fight.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
