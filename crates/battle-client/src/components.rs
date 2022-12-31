use battle::{pokemon::{remote::InitUnknownPokemon, ActivePosition}, pokedex::{Money, trainer::TrainerGroupId}};
use bevy::prelude::{Component, Color, Deref, DerefMut};
use pokengine::engine::text::MessagePage;
use serde::{Serialize, Deserialize};


#[derive(Component)]
pub struct BattleComponent;

#[derive(Component)]
pub struct TransformMultiplier(f32);

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct BattleTrainer {
    pub worth: Money,
    pub texture: TrainerGroupId,
    pub defeat: Vec<MessagePage<Color, ()>>,
}

#[derive(Component, Deref, DerefMut)]
pub struct BattleId<ID>(pub ID);

#[derive(Component, Deref, DerefMut)]
pub struct PlayerName(pub String);

#[derive(Component, Deref, DerefMut)]
pub struct Active<A>(pub Vec<Option<A>>);

#[derive(Component, Deref, DerefMut)]
pub struct Trainer<T>(pub T);

#[derive(Component)]
pub struct RemoteParty(pub Vec<Option<InitUnknownPokemon>>);

#[derive(Component, Deref, DerefMut)]
pub struct Select(pub ActivePosition);