use deps::hash::HashMap;
use firecore_audio_lib::serialized::SerializedMusicData;
use firecore_audio_lib::serialized::SerializedSoundData;

use crate::error::AddAudioError;

pub fn create() -> Result<(), AddAudioError> {
    let sdl = sdl2::init()?;
    start_context(&sdl, 2)?;
    drop(sdl);
    Ok(())
}


pub fn add_track(music_data: SerializedMusicData) -> Result<(), AddAudioError> {
        let music = from_bytes(music_data.bytes.into_boxed_slice())?;
        if let Some(map) = super::music::MUSIC_MAP.lock().as_mut() {
            map.insert(music_data.music.track, (music_data.music.data, super::MusicHandle(music)));
        }
        Ok(())
}

pub fn add_sound(sound_data: SerializedSoundData) -> Result<(), AddAudioError> {
    let sound = sdl2::mixer::Chunk::from_raw_buffer(sound_data.bytes.into_boxed_slice())?;
    if let Some(map) = super::sound::SOUND_MAP.lock().as_mut() {
        map.insert(sound_data.sound, super::SoundHandle(sound));
    }
    Ok(())
}

fn start_context(
    sdl: &sdl2::Sdl,
    num_sound_channels: i32,
) -> Result<(), AddAudioError> {
    let audio = sdl.audio()?;
    let timer = sdl.timer()?;

    init_audio(num_sound_channels)?;

    *super::music::MUSIC_MAP.lock() = Some(HashMap::new());
    *super::sound::SOUND_MAP.lock() = Some(HashMap::new());

    drop(timer);
    drop(audio);

    Ok(())
}

fn init_audio(num_sound_channels: i32) -> Result<(), AddAudioError> {
    use sdl2::mixer;
    // Load dynamic libraries.
    // Ignore formats that are not built in.
    let _ = mixer::init(
        mixer::InitFlag::MOD | mixer::InitFlag::OGG,
    )?;
    mixer::open_audio(
        mixer::DEFAULT_FREQUENCY,
        mixer::DEFAULT_FORMAT,
        mixer::DEFAULT_CHANNELS,
        1024,
    )?;
    // Sets the number of simultaneous sound effects channels
    // that are available.
    mixer::allocate_channels(num_sound_channels);
    Ok(())
}

fn from_bytes(buf: Box<[u8]>) -> Result<sdl2::mixer::Music<'static>, String> { // we do a little trolling
    let rw =
        unsafe { sdl2::sys::SDL_RWFromMem(buf.as_ptr() as *mut std::ffi::c_void, buf.len() as sdl2::libc::c_int) };

    if rw.is_null() {
        return Err(sdl2::get_error());
    }

    let raw = unsafe { sdl2::sys::mixer::Mix_LoadMUS_RW(rw, 0) };
    if raw.is_null() {
        Err(sdl2::get_error())
    } else {
        Ok(sdl2::mixer::Music {
            raw: raw,
            owned: true,
            _marker: std::marker::PhantomData,
        })
    }
}