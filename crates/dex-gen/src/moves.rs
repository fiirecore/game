use firecore_battle::pokedex::moves::MoveId;
use pokerust::Id;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{ops::Range, sync::Arc};

use battle::{
    default_engine::moves::{MoveExecution, MoveUse},
    moves::damage::DamageKind,
    pokedex::{
        ailment::{Ailment, AilmentLength},
        moves::{Move, MoveCategory, MoveTarget},
        pokemon::stat::StatType,
    },
    pokemon::stat::BattleStatType,
};

pub type Execution = hashbrown::HashMap<MoveId, MoveExecution>;

pub fn generate(client: Arc<pokerust::Client>, range: Range<i16>) -> Vec<Move> {
    range
        .into_par_iter()
        .map(|index| get_move(index, &client))
        .collect()
}

pub fn generate_battle(client: Arc<pokerust::Client>, range: Range<i16>) -> Execution {
    range
        .into_par_iter()
        .map(|m| {
            let move_ = client.get::<pokerust::Move, i16>(m).unwrap_or_else(|err| {
                eprintln!("Could not get move from id {} with error {}", m, err);
                panic!()
            });

            let id = move_
                .name
                .parse()
                .expect("Could not parse move name into ASCII string!");

            (id, get_move_execution(&move_))
        })
        .collect()
}

fn get_move(index: i16, client: &pokerust::Client) -> Move {
    let mut move_ = client
        .get::<pokerust::Move, i16>(index)
        .unwrap_or_else(|err| {
            eprintln!("Could not get move from id {} with error {}", index, err);
            panic!()
        });

    let name = move_.names.remove(7).name;

    let id = move_
        .name
        .parse()
        .expect("Could not parse move name into ASCII string!");

    crate::capitalize_first(&mut move_.type_.name);
    crate::capitalize_first(&mut move_.damage_class.name);

    println!("Creating move entry for: {}", name);

    Move {
        id,
        pp: move_
            .pp
            .unwrap_or_else(|| panic!("Could not get PP for pokemon move {}", name)),
        name,
        category: category_from_id(move_.damage_class.id()),
        pokemon_type: crate::type_from_id(move_.type_.id()),
        power: move_.power,
        accuracy: move_.accuracy,
        priority: move_.priority,
        target: target_from_id(move_.target.id()),
        contact: false,
        crit_rate: move_
            .meta
            .as_ref()
            .map(|meta| meta.crit_rate)
            .unwrap_or_default(),
        // world: is_world_move(&move_),
    }
}

fn category_from_id(id: i16) -> MoveCategory {
    match id {
        2 => MoveCategory::Physical,
        3 => MoveCategory::Special,
        1 => MoveCategory::Status,
        _ => panic!("Could not get move category from id \"{}\"", id),
    }
}

fn get_move_execution(move_: &pokerust::Move) -> MoveExecution {
    match move_.name.as_str() {
        "false-swipe" => MoveExecution::Script,
        _ => {
            let actions = get_move_actions(move_);
            match actions.is_empty() {
                true => MoveExecution::None,
                false => MoveExecution::Actions(actions),
            }
        }
    }
}

fn get_move_actions(move_: &pokerust::Move) -> Vec<MoveUse> {
    let mut usages = Vec::with_capacity(1);

    if let Some(power) = move_.power {
        usages.push(MoveUse::Damage(DamageKind::Power(power)))
    }

    // metadata

    if let Some(metadata) = move_.meta.as_ref() {
        // flinch check

        if metadata.flinch_chance != 0 {
            let flinch = vec![MoveUse::Flinch];
            usages.push(if metadata.flinch_chance == 100 {
                MoveUse::Flinch
            } else {
                MoveUse::Chance(flinch, metadata.flinch_chance)
            });
        }

        // drain check

        if metadata.drain != 0 {
            if let Some(MoveUse::Damage(kind)) = usages.get(0) {
                usages[0] = MoveUse::Drain(*kind, metadata.drain);
            }
        }

        // status effect check

        if !matches!(metadata.ailment.id(), -1 | 0) {
            let range = status_range(metadata.min_turns, metadata.max_turns);

            if let Some(ailment) = match metadata.ailment.id() {
                1 => Some(Ailment::Paralysis),
                2 => Some(Ailment::Sleep),
                3 => Some(Ailment::Freeze),
                4 => Some(Ailment::Burn),
                5 => Some(Ailment::Poison),
                id => {
                    eprintln!("Could not get ailment #{}", id);
                    None
                }
            } {
                usages.push(MoveUse::Ailment(
                    Some((ailment, range)),
                    metadata.ailment_chance,
                ));
            }
        }

        // stat stage check

        if !move_.stat_changes.is_empty() {
            let stat_changes = move_
                .stat_changes
                .iter()
                .map(|stat| MoveUse::Stat(get_stat_type(stat.stat.id(), &move_.name), stat.change));

            if matches!(metadata.stat_chance, 0 | 100) {
                usages.extend(stat_changes);
            } else {
                usages.push(MoveUse::Chance(
                    stat_changes.collect(),
                    metadata.stat_chance,
                ));
            }
        }
    }

    // if usages.is_empty() {
    //     usages.push(MoveUse::Todo)
    // }

    usages
}

// /// 15 = Cut, 19 = Fly, 57 = Surf, 70 = Strength, 127 = Waterfall, 249 = Rock Smash
// fn is_world_move(move_: &pokerust::Move) -> bool {
//     match move_.id {
//         15 | 19 | 57 | 70 | 127 | 249 => true,
//         _ => false,
//     }
// }

fn status_range(min_turns: Option<u8>, max_turns: Option<u8>) -> AilmentLength {
    match min_turns.zip(max_turns) {
        Some((min, max)) => AilmentLength::Temporary(min, max),
        None => AilmentLength::Permanent,
    }
}

fn get_stat_type(id: i16, name: &str) -> BattleStatType {
    match id {
        1 => BattleStatType::Basic(StatType::Health),
        2 => BattleStatType::Basic(StatType::Attack),
        3 => BattleStatType::Basic(StatType::Defense),
        4 => BattleStatType::Basic(StatType::SpAttack),
        5 => BattleStatType::Basic(StatType::SpDefense),
        6 => BattleStatType::Basic(StatType::Speed),
        7 => BattleStatType::Accuracy,
        8 => BattleStatType::Evasion,
        id => {
            eprintln!("Move {} has unknown battle stat type id {}", name, id);
            panic!()
        }
    }
}

fn target_from_id(target: i16) -> MoveTarget {
    match target {
        3 => MoveTarget::Ally,
        5 => MoveTarget::UserOrAlly,
        13 => MoveTarget::UserAndAllies,
        9 => MoveTarget::AllOtherPokemon,
        10 => MoveTarget::Any,
        11 => MoveTarget::AllOpponents,
        15 => MoveTarget::Allies,
        // 13 => MoveTarget::UserOrAllies,
        7 => MoveTarget::User,
        14 => MoveTarget::AllPokemon,
        _ => MoveTarget::None,
    }
}
