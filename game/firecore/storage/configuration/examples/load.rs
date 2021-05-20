use std::ops::Deref;

use firecore_storage::{
    {load, get},
    macroquad::{self, prelude::{info, collections::storage::store}}
};

use firecore_configuration::Configuration;

#[macroquad::main("Window")]
async fn main() {

    info!("Attempting to load configuration!");

    store(load::<Configuration>().await);

    info!("Loaded configuration!");

    info!("{:?}", get::<Configuration>().unwrap().deref());

}