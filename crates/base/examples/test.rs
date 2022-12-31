use bevy::prelude::*;
use bevy_egui::*;
use engine::{
    controls::Control,
    gui::{MessageBox, GlobalMessageState},
    text::{MessagePage, MessageState, MessageStates},
};
use firecore_base as engine;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(engine::controls::ControlsPlugin)
        .insert_resource({
            let page = MessagePage {
                lines: vec![
                    "Test Pagé Test Page".to_owned(),
                    "Pagé Test Page Test".to_owned(),
                ],
                wait: None,
                color: Some(Color::RED),
                theme: (),
            };
            let page2 = MessagePage {
                lines: vec![
                    "Test Pagé Test Page 2".to_owned(),
                    "Pagé Test Page Test 2".to_owned(),
                ],
                wait: Some(1.0),
                color: Some(Color::YELLOW),
                theme: (),
            };

            GlobalMessageState(MessageStates::Running(MessageState {
                pages: vec![page, page2],
                ..Default::default()
            }))
        })
        .add_system(ui)
        .run();
}

// fn start(audio: Res<Audio>, assets: Res<AssetServer>) {
//     let music = assets.load("");
//     let sink = audio.play_with_settings(music, Default::default());

// }

fn ui(
    input: Res<Input<Control>>,
    time: Res<Time>,
    mut egui: ResMut<EguiContext>,
    mut state: ResMut<GlobalMessageState>,
) {
    MessageBox::ui(&input, time.delta_seconds(), egui.ctx_mut(), &mut state);
    //     if !state.is_running() {
    //         app.exit();
    //     }
}
