use std::sync::Arc;

use bevy::{
    audio::AudioSink,
    prelude::{Assets, Audio, AudioSource, EventReader, Handle, Plugin, Res, ResMut, Resource},
    utils::HashMap,
};

pub use firecore_audio::*;

pub enum AudioEvent {
    PlayMusic(MusicId),
    StopMusic,
    PlaySound(SoundId),
    // SetVolume(f32),
}

#[derive(Default, Resource)]
pub struct CurrentMusic(Option<MusicHandle>);

pub struct MusicHandle(pub MusicId, pub Handle<AudioSink>);

#[derive(Resource)]
pub struct AudioSources {
    pub music: HashMap<MusicId, Handle<AudioSource>>,
    pub sounds: HashMap<SoundId, Handle<AudioSource>>,
    pub volume: f32,
}

impl AudioSources {
    pub fn insert_music(
        &mut self,
        sources: &mut Assets<AudioSource>,
        id: MusicId,
        bytes: impl Into<Arc<[u8]>>,
    ) {
        self.music.insert(
            id,
            sources.add(AudioSource {
                bytes: bytes.into(),
            }),
        );
    }

    pub fn insert_sound(
        &mut self,
        sources: &mut Assets<AudioSource>,
        id: SoundId,
        bytes: impl Into<Arc<[u8]>>,
    ) {
        self.sounds.insert(
            id,
            sources.add(AudioSource {
                bytes: bytes.into(),
            }),
        );
    }
}

impl Default for AudioSources {
    fn default() -> Self {
        Self {
            music: Default::default(),
            sounds: Default::default(),
            volume: 0.5,
        }
    }
}

fn audio_listener(
    audio: Res<Audio>,
    sources: Res<AudioSources>,
    mut sinks: ResMut<Assets<AudioSink>>,
    mut current: ResMut<CurrentMusic>,
    mut reader: EventReader<AudioEvent>,
) {
    for event in reader.iter() {
        match event {
            AudioEvent::PlayMusic(id) => {
                if Some(id) != current.0.as_ref().map(|m| &m.0) {
                    stop_music(&mut sinks, &mut current.0);
                }
                if let Some(source) = sources.music.get(id) {
                    current.0 = Some(MusicHandle(
                        *id,
                        audio.play_with_settings(
                            source.clone(),
                            bevy::prelude::PlaybackSettings {
                                repeat: true,
                                volume: sources.volume,
                                speed: 1.0,
                            },
                        ),
                    ));
                }
            }
            AudioEvent::StopMusic => stop_music(&mut sinks, &mut current.0),
            AudioEvent::PlaySound(sound) => {
                if let Some(source) = sources.sounds.get(sound) {
                    audio.play(source.clone());
                }
            } // AudioEvent::SetVolume(_) => todo!(),
        }
    }
}

fn stop_music(sinks: &mut Assets<AudioSink>, current: &mut Option<MusicHandle>) {
    if let Some(current) = current.take() {
        if let Some(sink) = sinks.remove(current.1) {
            sink.stop();
        }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<AudioEvent>()
            .init_resource::<CurrentMusic>()
            .init_resource::<AudioSources>();

        app.add_system(audio_listener);
    }
}
