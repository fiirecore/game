use firecore_audio_lib::serialized::SerializedSoundData;
use firecore_audio_lib::sound::Sound;

pub fn play_sound(sound: &Sound) -> Result<(), crate::error::PlayAudioError> {
    super::backend::sound::play_sound(&sound)
}

pub fn add_sound(sound_data: SerializedSoundData) -> Result<(), crate::error::AddAudioError> {
    super::backend::context::add_sound(sound_data)
}