use notan::{
    egui,
    prelude::{App, Plugins},
};

use text::MessageStates;

use crate::controls::{pressed, Control};

pub struct MessageBox;

impl MessageBox {
    pub fn ui<C: Clone + Into<[f32; 4]>, T: Default>(
        app: &App,
        plugins: &mut Plugins,
        egui: &egui::Context,
        state: &mut MessageStates<C, T>,
    ) {
        if state.is_running() {
            egui::Window::new("Message Box")
                .title_bar(false)
                .show(egui, |ui| {
                    if let MessageStates::Running(message) = state {
                        if let Some(page) = message.pages.get(message.page) {
                            if page.lines.len() == 0 {
                                message.page += 1;
                                return;
                            }

                            if !message.waiting {
                                match (message.text as usize)
                                    < page
                                        .lines
                                        .get(message.line)
                                        .map(String::len)
                                        .unwrap_or_default()
                                {
                                    true => message.text += app.timer.delta_f32() * 30.0,
                                    false => {
                                        message.text = 0.0;
                                        if message.line < page.lines.len() - 1 {
                                            message.line += 1;
                                        } else {
                                            message.waiting = true;
                                        }
                                    }
                                }
                            }

                            for line in page
                                .lines
                                .get(..message.line)
                                .into_iter()
                                .flat_map(|i| i.iter())
                                .rev()
                            {
                                ui.label(line);
                            }

                            if let Some(line) = page.lines.get(message.line) {
                                let len = message.text as usize;

                                let (string, finished) = line
                                    .char_indices()
                                    .nth(len)
                                    .filter(|_| !message.waiting)
                                    .map(|(len, ..)| line.get(..len).map(|l| (l, false)))
                                    .flatten()
                                    .unwrap_or_else(|| {
                                        (line.as_str(), message.line + 1 >= page.lines.len())
                                    });

                                ui.label(string);

                                if finished || (len != 0 && string.is_empty()) {
                                    match page.wait {
                                        Some(wait) => {
                                            message.wait += app.timer.delta_f32();
                                            if message.wait.abs() >= wait.abs() {
                                                message.waiting = false;
                                                match message.page + 1 >= message.pages() {
                                                    true => {
                                                        *state = match message.cooldown {
                                                            Some(cooldown) => {
                                                                MessageStates::Finished(cooldown)
                                                            }
                                                            None => MessageStates::None,
                                                        };
                                                        return;
                                                    }
                                                    false => {
                                                        message.page += 1;
                                                        message.reset_page();
                                                    }
                                                }
                                            }
                                        }
                                        None => {
                                            if ui.button("Next").clicked()
                                                || pressed(app, plugins, Control::A)
                                            {
                                                message.waiting = false;
                                                if message.page + 1 >= message.pages() {
                                                    *state = match message.cooldown {
                                                        Some(cooldown) => {
                                                            MessageStates::Finished(cooldown)
                                                        }
                                                        None => MessageStates::None,
                                                    };
                                                } else {
                                                    message.page += 1;
                                                    message.reset_page();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            *state = match message.cooldown {
                                Some(cooldown) => MessageStates::Finished(cooldown),
                                None => MessageStates::None,
                            };
                        }
                    }
                });
        }
    }
}
