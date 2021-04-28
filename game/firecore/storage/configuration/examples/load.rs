use std::ops::Deref;

use firecore_storage::{
    {store, get},
    macroquad::{self, prelude::info}
};

use firecore_configuration::Configuration;

#[macroquad::main("Window")]
async fn main() {

    info!("Attempting to load configuration!");

    store::<Configuration>().await;

    info!("Loaded configuration!");

    info!("{:?}", get::<Configuration>().unwrap().deref());

}