use std::io::Write;

use firecore_audio_lib::serialized::*;

pub fn compile<P: AsRef<std::path::Path>>(music_folder: P, output_file: P) {
    let music_folder = music_folder.as_ref();
    let output_file = output_file.as_ref();

    let mut music = Vec::with_capacity(20);

    for dir in std::fs::read_dir(music_folder).unwrap_or_else(|err| panic!("Could not read music directory with error {}", err)) {
        match dir.map(|dir| dir.path()) {
            Ok(path) => {
                if path.is_file() {
                    let content = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("Could not read file at {:?} to string with error {}", path, err));
                    let ser_music_file: SerializedMusicFile = ron::from_str(&content).unwrap_or_else(|err| panic!("Could not deserialize file at {:?} with error {}", path, err));
                    let music_bytes = std::fs::read(music_folder.join(&ser_music_file.file)).unwrap_or_else(|err| panic!("Could not get music file for {} with error {}", ser_music_file.music.name, err));
                    music.push(SerializedMusicData {
                        bytes: music_bytes,
                        music: ser_music_file.music,
                    });
                }
            },
            Err(err) => {
                eprintln!("Could not read entry under music directory with error {}", err);
            }
        }
    }

    for curr_music in music.iter().map(|data| &data.music) {
        for music in music.iter().map(|data| &data.music).filter(|music| music.name.ne(&curr_music.name)) {
            if music.track.eq(&curr_music.track) {
                panic!("Music {} and {} have equal track IDs! Aborting!", music.name, curr_music.name);
            }
        }
    }

    let data = firecore_audio_lib::serialized::SerializedAudio {
        music,
        sounds: Vec::new(),
    };

    if let Some(parent) = std::path::Path::new(output_file).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| panic!("Could not create directory for output file with error {}", err));
        }
    }

    std::fs::File::create(output_file)
        .unwrap_or_else(|err| panic!("Could not create output file with error {}", err))
            .write(&postcard::to_allocvec(&data).unwrap_or_else(|err| panic!("Could not serialize audio data with error {}", err)))
                .unwrap_or_else(|err| panic!("Could not write to output file with error {}", err));
}