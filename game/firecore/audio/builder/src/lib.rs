extern crate firecore_audio as audio;

use std::{io::Write, path::{Path, PathBuf}, fs::{File, read, read_dir, read_to_string, create_dir_all}};

use audio::serialized::*;

pub fn compile<P: AsRef<Path>>(music_folder: P, output_file: P) {
    let music_folder = music_folder.as_ref();
    let output_file = output_file.as_ref();

    let mut music = Vec::with_capacity(20);

    for dir in read_dir(music_folder).unwrap_or_else(|err| panic!("Could not read music directory with error {}", err)) {
        match dir.map(|dir| dir.path()) {
            Ok(path) => {
                if path.is_file() {
                    let content = read_to_string(&path).unwrap_or_else(|err| panic!("Could not read file at {:?} to string with error {}", path, err));
                    let ser_music_file: SerializedMusicFile = ron::from_str(&content).unwrap_or_else(|err| panic!("Could not deserialize file at {:?} with error {}", path, err));
                    let music_bytes = read(music_folder.join(&ser_music_file.file)).unwrap_or_else(|err| panic!("Could not get music file for {} with error {}", ser_music_file.music.name, err));
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
            if music.track == curr_music.track {
                panic!("Music {} and {} have equal track IDs! Aborting!", music.name, curr_music.name);
            }
        }
    }

    let data = SerializedAudio {
        music,
        sounds: Vec::new(),
    };

    if let Some(parent) = Path::new(output_file).parent() {
        if !parent.exists() {
            create_dir_all(parent).unwrap_or_else(|err| panic!("Could not create directory for output file with error {}", err));
        }
    }

    File::create(output_file)
        .unwrap_or_else(|err| panic!("Could not create output file with error {}", err))
            .write_all(&firecore_dependencies::ser::serialize(&data).unwrap_or_else(|err| panic!("Could not serialize audio data with error {}", err)))
                .unwrap_or_else(|err| panic!("Could not write to output file with error {}", err));
}