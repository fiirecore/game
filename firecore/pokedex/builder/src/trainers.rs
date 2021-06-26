use std::{
    fs::{read, read_dir},
    path::Path,
};

use pokedex::serialize::SerializedTrainers;

pub fn get_trainers(trainer_dir: impl AsRef<Path>) -> SerializedTrainers {
    read_dir(trainer_dir)
        .unwrap_or_else(|err| panic!("Could not read trainer directory with error {}", err))
        .flatten()
        .map(|d| {
            (
                d.file_name()
                    .to_string_lossy()
                    .split('.')
                    .next()
                    .unwrap_or_else(|| {
                        panic!(
                            "Could not read file name of trainer texture at {:?}",
                            d.file_name()
                        )
                    })
                    .parse()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Cannot parse file name for trainer texture at {:?} with error {}",
                            d.file_name(),
                            err
                        )
                    }),
                read(d.path()).unwrap_or_else(|err| {
                    panic!("Could not read trainer texture entry with error {}", err)
                }),
            )
        })
        .collect()
}
