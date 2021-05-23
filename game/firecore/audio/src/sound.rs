use firecore_audio_lib::serialized::SerializedSoundData;
use firecore_audio_lib::sound::Sound;

pub fn play_sound(sound: &Sound) -> Result<(), crate::error::PlayAudioError> {
    // macroquad::prelude::info!("Playing sound {:?}", sound);
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
    super::backend::sound::play_sound(&sound)?;
    Ok(())
}

pub fn add_sound(sound_data: SerializedSoundData) -> Result<(), crate::error::AddAudioError> {
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
    super::backend::context::add_sound(sound_data)?;
    Ok(())
}