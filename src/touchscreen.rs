use crate::engine::{
    controls::{context::ControlsContext, Control},
    egui, Plugins,
};

pub struct Touchscreen;

impl Touchscreen {
    pub fn ui(egui: &egui::Context, plugins: &mut Plugins) {
        if let Some(mut controls) = plugins.get_mut::<ControlsContext>() {
            fn create_button(ui: &mut egui::Ui, name: &str) -> egui::Response {
                ui.centered_and_justified(|ui| ui.add(egui::Button::new(name)))
                    .response
            }

            fn button(controls: &mut ControlsContext, ui: &mut egui::Ui, control: Control) {
                if create_button(ui, control.name()).hovered() {
                    if !controls.touchscreen.down.contains(&control) {
                        controls.touchscreen.pressed.insert(control);
                    }
                } else {
                    controls.touchscreen.remove(control);
                }
            }

            controls.touchscreen.update();

            egui::Window::new("Buttons")
                .anchor(egui::Align2::LEFT_BOTTOM, [0.0; 2])
                .resizable(false)
                .title_bar(false)
                .resizable(false)
                .show(egui, |ui| {
                    egui::Grid::new("ButtonGrid").show(ui, |ui| {
                        button(&mut controls, ui, Control::A);
                        button(&mut controls, ui, Control::B);

                        ui.end_row();

                        button(&mut controls, ui, Control::Start);
                        button(&mut controls, ui, Control::Select);
                    })
                });

            egui::Window::new("D-pad")
                .anchor(egui::Align2::RIGHT_BOTTOM, [0.0; 2])
                .resizable(false)
                .title_bar(false)
                .resizable(false)
                .show(egui, |ui| {
                    egui::Grid::new("DPadGrid").show(ui, |ui| {
                        create_button(ui, "");
                        button(&mut controls, ui, Control::Up);

                        ui.end_row();

                        button(&mut controls, ui, Control::Left);
                        create_button(ui, "");
                        button(&mut controls, ui, Control::Right);

                        ui.end_row();

                        create_button(ui, "");
                        button(&mut controls, ui, Control::Down);
                    })
                });
        }
    }
}
