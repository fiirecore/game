use std::sync::Arc;

use std::fs;

use std::process;

use std::process::Stdio;

use tempfile::TempDir;

pub fn get_cry(tempdir: Arc<TempDir>, pokemon: Arc<String>) -> Vec<u8> {
    let pokemon = if pokemon.as_str() == "unown/e" {
        Arc::new("unown".to_string())
    } else {
        pokemon
    };
    let response = attohttpc::get(&format!("https://raw.githubusercontent.com/pret/pokefirered/master/sound/direct_sound_samples/cry_{}.aif", pokemon)).send().unwrap_or_else(|err| panic!("Could not get web response for cry of {}. Error: {}", pokemon, err));
    let bytes = response
        .bytes()
        .unwrap_or_else(|err| panic!("Could not get cry bytes for {} with error {}", pokemon, err));
    let temp_path = tempdir.path();
    let temp_cry = temp_path.join(format!("{}-temp_cry.aif", pokemon));
    let cry = temp_path.join(format!("{}-cry.ogg", pokemon));
    fs::write(&temp_cry, &bytes).unwrap_or_else(|err| {
        panic!(
            "Could not write temporary cry file at {:?} for {} with error {}",
            &temp_cry, pokemon, err
        )
    });
    let mut command = process::Command::new("ffmpeg");
    command.stdout(Stdio::null()); //.kill_on_drop(true);
    command.arg("-i").arg(&temp_cry).arg(&cry);
    command.output().unwrap_or_else(|err| {
        panic!(
            "Could not execute ffmpeg for {} with error {}",
            pokemon, err
        )
    });
    fs::read(cry)
        .unwrap_or_else(|err| panic!("Could not read cry file for {} with error {}", pokemon, err))
}
