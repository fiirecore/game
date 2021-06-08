use pokedex::moves::target::MoveTargetInstance;

use crate::battle::{
    client::BattleClient,
    pokemon::{
        view::{BattlePartyKnown, BattlePartyTrait, BattlePartyUnknown, PokemonUnknown},
        BattleMove,
    },
    BattleType,
};

#[derive(Default)]
pub struct BattlePlayerAi {
    user: BattlePartyKnown,
    opponent: BattlePartyUnknown,

    moves: Option<Vec<BattleMove>>,
}

impl BattleClient for BattlePlayerAi {

    fn user(&mut self, _type: BattleType, user: BattlePartyKnown) {
        self.user = user;
    }

    fn opponents(&mut self, opponent: BattlePartyUnknown) {
        self.opponent = opponent;        
    }

    fn add_unknown(&mut self, index: usize, unknown: PokemonUnknown) {
        self.opponent.add(index, unknown);
    }

    fn start_select(&mut self) {
        // note: does not use PP
        use crate::battle::BATTLE_RANDOM;
        self.moves = Some(
            self.user
                .active
                .iter()
                .flat_map(|index| (*index).map(|index| &self.user.pokemon[index]))
                .map(|pokemon| {
                    // crashes when moves run out
                    let moves: Vec<usize> = pokemon
                        .moves
                        .iter()
                        .enumerate()
                        .filter(|(_, instance)| instance.pp != 0)
                        .map(|(index, _)| index)
                        .collect();

                    BattleMove::Move(
                        moves[BATTLE_RANDOM.gen_range(0, moves.len())],
                        MoveTargetInstance::opponent(
                            self.opponent.id,
                            BATTLE_RANDOM.gen_range(0, self.opponent.active.len()),
                        ),
                    )
                })
                .collect(),
        );
    }

    fn wait_select(&mut self) -> Option<Vec<BattleMove>> {
        self.moves.take()
    }

    fn start_moves(&mut self, _: Vec<crate::battle::pokemon::BattleClientActionInstance>) {}

    fn wait_faint(&mut self, active: usize) -> Option<usize> {
        if let Some(pokemon) = self.user.active_mut(active) {
            pokemon.set_hp(0.0);
        }

        let available: Vec<usize> = self
            .user
            .pokemon
            .iter()
            .enumerate()
            .filter(|(index, pokemon)| active.ne(index) && !pokemon.fainted())
            .map(|(index, _)| index)
            .collect(); // To - do: use position()

        let r = if available.is_empty() {
            deps::log::debug!("AI has no inactive pokemon!");
            None
        } else {
            Some(available[crate::battle::BATTLE_RANDOM.gen_range(0, available.len())])
        };

        self.user.replace(active, r);

        r
    }

    fn opponent_faint_replace(&mut self, active: usize, new: Option<usize>) {
        self.opponent.replace(active, new)
    }

    fn wait_finish_turn(&mut self) -> bool {
        true
    }

    fn should_forfeit(&mut self) -> bool {
        false
    }

}
