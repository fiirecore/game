use firecore_game as game;
use firecore_battle_game::BattleManager;
use game::{gui::{bag::BagGui, party::PartyGui}, log::{info, warn}, tetra::{Context, Result, State, graphics::{self, Color, scaling::{ScalingMode, ScreenScaler}}, time::{self, Timestep}}, world::battle::random_wild_battle};
use std::rc::Rc;

fn main() -> Result {
    game::init::logger();
    game::tetra::ContextBuilder::new(
        "Battle Test",
        game::util::WIDTH as i32 * 3,
        game::util::HEIGHT as i32 * 3,
    )
    .timestep(Timestep::Variable)
    .show_mouse(true)
    .build()?
    .run(BattleState::new)
}

struct BattleState {
    scaler: ScreenScaler,
    manager: BattleManager,
}

impl BattleState {
    pub fn new(ctx: &mut Context) -> Result<Self> {
        let party = Rc::new(PartyGui::new(ctx));
        let bag = Rc::new(BagGui::new(ctx));
        game::init::configuration()?;
        game::init::text(
            ctx, 
            game::deps::ser::deserialize(include_bytes!(
                "../../../game/build/data/fonts.bin"
            ))
            .unwrap(),
        )?;
        game::init::pokedex(
            ctx,
            game::deps::ser::deserialize(include_bytes!(
                "../../../game/build/data/dex.bin"
            ))
            .unwrap(),
        )?;
        game::init::seed_random(game::util::date());
        game::storage::init();
        unsafe {
            game::storage::PLAYER_SAVES.as_mut().unwrap().select(0)
        }
        Ok(Self {
            scaler: ScreenScaler::with_window_size(ctx, game::util::WIDTH as _, game::util::HEIGHT as _, ScalingMode::ShowAll)?,
            manager: BattleManager::new(ctx, party, bag),
        })
    }
}

impl State for BattleState {
    fn begin(&mut self, _ctx: &mut Context) -> Result {
        let mut battle_entry = None;
        random_wild_battle(&mut battle_entry, 2);
        if !self.manager.battle(battle_entry.unwrap()) {
            warn!("Could not create battle with player data!");
            panic!("Player data: {:?}", game::storage::data());
        }
        info!("Successfully loaded.");
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result {
        self.manager
            .update(ctx, time::get_delta_time(ctx).as_secs_f32(), false);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);
        self.manager.draw(ctx);
        graphics::reset_transform_matrix(ctx);
        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);
        Ok(())
    }
}
