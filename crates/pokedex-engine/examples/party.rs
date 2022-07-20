use std::rc::Rc;

use firecore_pokedex_engine::{
    engine::{
        egui::EguiPluginSugar,
        notan::{
            self,
            prelude::{App, Graphics, Plugins},
            AppState,
        },
        utils::Entity,
    },
    gui::party::PartyGui,
    pokedex::{
        item::Item,
        moves::Move,
        pokemon::{
            owned::{OwnedPokemon, SavedPokemon},
            Pokemon,
        },
        BasicDex, Dex,
    },
    PokedexClientData,
};

fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(notan::egui::EguiConfig)
        .update(State::update)
        .draw(State::draw)
        .build()
}

#[derive(AppState)]
struct State {
    party: PartyGui<Rc<PokedexClientData>>,
    pokemon: Vec<OwnedPokemon<Rc<Pokemon>, Rc<Move>, Rc<Item>>>,
}

impl State {
    fn new(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins) -> Self {
        firecore_pokedex_engine::engine::setup(plugins);

        // deserialize bin files

        // let (pdex, mdex, idex) = ;

        let (pokedex, movedex, itemdex): (
            BasicDex<Pokemon, Rc<_>>,
            BasicDex<Move, Rc<_>>,
            BasicDex<Item, Rc<_>>,
        ) = firecore_storage::from_bytes(include_bytes!("./dex.bin")).unwrap();

        let data = Rc::new(
            firecore_pokedex_engine::PokedexClientData::build(
                app,
                plugins,
                gfx,
                firecore_storage::from_bytes(include_bytes!("./dex_engine.bin")).unwrap(),
            )
            .unwrap(),
        );

        let mut party = PartyGui::new(data);

        let mut pokemon = Vec::new();

        for i in 0..7 {
            pokemon.push(SavedPokemon {
                pokemon: (i + 2) * 24,
                level: 20,
                ..Default::default()
            });
        }

        println!("{}", pokedex.len());

        dbg!(&pokemon);

        let pokemon = pokemon
            .into_iter()
            .flat_map(|p| p.init(&mut rand::thread_rng(), &pokedex, &movedex, &itemdex))
            .collect::<Vec<_>>();

        dbg!(&pokemon);

        party.spawn();

        Self { party, pokemon }
    }

    fn update(app: &mut App, state: &mut State) {
        if !state.party.alive() {
            app.exit();
        }
    }

    fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
        gfx.render(&plugins.egui(|ctx| {
            state.party.ui(app, ctx, &mut state.pokemon);
        }));
    }
}
