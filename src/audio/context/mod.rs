pub mod music;
pub mod sound;

fn stop_instance(audio: impl std::fmt::Display, mut instance: kira::instance::handle::InstanceHandle) {
    if let Err(err) = instance.stop(kira::instance::StopInstanceSettings::default().fade_tween(kira::parameter::tween::Tween::linear(0.75))) {
        macroquad::prelude::warn!("Problem stopping audio instance {} with error {}", audio, err);
    }
}