use bevy::prelude::*;
use iyes_loopless::prelude::*;
use pokengine::texture::TrainerGroupTextures;

use crate::{
    components::{BattleComponent, BattleTrainer, PlayerName, RemoteParty},
    resources::{BattleBackground, CurrentRemote},
    BattleClientState,
};

#[derive(Component)]
pub struct Transition;

fn spawn_scene(
    mut commands: Commands,
    remote: Res<CurrentRemote>,
    trainers: Res<TrainerGroupTextures>,
    textures: Res<BattleBackground>,
    query: Query<(Option<&PlayerName>, Option<&BattleTrainer>, &RemoteParty)>,
) {

    info!("spawn scene");

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            scaling_mode: bevy::render::camera::ScalingMode::Auto { min_width: 240.0, min_height: 160.0 },
            scale: 2.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            texture: textures.background.clone(),
            ..Default::default()
        },
        BattleComponent,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.2 * -240.0, 0.2 * -240.0, 0.1)),
            texture: textures.floor.clone(),
            ..Default::default()
        },
        BattleComponent,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.8 * 240.0, 0.8, 0.1)),
            texture: textures.floor.clone(),
            ..Default::default()
        },
        BattleComponent,
    ));

    let mut name = None;

    if let Some(data) = remote.data {
        if let Ok((pname, trainer, party)) = query.get(data) {
            name = pname.map(|s| &s.0);
    
            if let Some(trainer) = trainer {
                if let Some(image) = trainers.get(&trainer.texture) {
                    let mut space = match remote.space {
                        Some(e) => commands.entity(e),
                        None => commands.spawn(BattleComponent),
                    };
                    space.insert(
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(0.6, 0.6, 0.2)),
                            texture: image.clone(),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(
            Some(BattleClientState::Opening),
            spawn_scene, // .with_system(system),
        );
    }
}
