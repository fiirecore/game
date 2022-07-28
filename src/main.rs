extern crate firecore_battle_engine as battlecli;
extern crate firecore_event as event;
extern crate firecore_world_engine as worldcli;

pub(crate) use battlecli::battle::pokedex;
pub(crate) use battlecli::pokengine;
pub(crate) use battlecli::pokengine::engine;

use state::StateManager;

mod battle_wrapper;
mod world_wrapper;

mod command;
mod config;
mod load;
mod random;
mod saves;
mod state;
mod touchscreen;

const TITLE: &str = "Pokemon PC Edition";
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const PUBLISHER: &str = "fiirecore";
const APPLICATION: &str = env!("CARGO_PKG_NAME");

const WIDTH: f32 = 240.0;
const HEIGHT: f32 = 160.0;
const SCALE: f32 = 3.0;

use engine::notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    use engine::notan;
    notan::init_with(run)
        .add_config(notan::egui::EguiConfig)
        .add_config(notan::draw::DrawConfig)
        .add_config(notan::log::LogConfig::debug())
        .add_loader(load::asset_loader())
        .update(update)
        .draw(draw)
        .add_config(WindowConfig {
            title: TITLE.to_string(),
            width: (WIDTH * SCALE) as _,
            height: (HEIGHT * SCALE) as _,
            min_size: Some((WIDTH as _, HEIGHT as _)),
            vsync: true,
            canvas_auto_resolution: true,
            ..Default::default()
        })
        .build()?;
    Ok(())
}

fn run(assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) -> StateManager {
    try_run(assets, gfx, plugins).unwrap()
}

fn try_run(
    assets: &mut Assets,
    gfx: &mut Graphics,
    plugins: &mut Plugins,
) -> Result<StateManager, String> {
    engine::setup(plugins);
    let load = load::LoadData::load(assets)?;
    StateManager::try_new(gfx, load)
}

fn update(app: &mut App, plugins: &mut Plugins, state: &mut StateManager) {
    state.update(app, plugins)
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut StateManager) {
    state.draw(app, plugins, gfx);
    app.window().request_frame();
}
