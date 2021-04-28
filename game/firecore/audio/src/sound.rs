use firecore_audio_lib::serialized::SerializedSoundData;
use firecore_audio_lib::sound::Sound;

pub fn play_sound(sound: Sound) -> Result<(), crate::error::PlayAudioError> {
    // macroquad::prelude::info!("Playing sound {:?}", sound);
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
    super::backend::kira::sound::play_sound(&sound)?;
    Ok(())
}

pub fn add_sound(sound_data: SerializedSoundData) -> Result<(), crate::error::AddAudioError> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
    super::backend::kira::context::add_sound(sound_data)?;
    Ok(())
}