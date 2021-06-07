use game::{
    util::{Entity, Reset},
    tetra::{Context, input::{self, Key}},
    game::CommandResult,
};

#[derive(Default)]
pub struct Console {

    pub alive: bool,
    command: String,

}

impl Console {

    pub fn update(&mut self, ctx: &Context) -> Option<CommandResult> {
        match self.alive {
            true => {
                if input::is_key_pressed(ctx, Key::Slash) || input::is_key_pressed(ctx, Key::Escape) {
                    self.despawn();
                }
                if input::is_key_pressed(ctx, Key::Backspace) {
                    self.command.pop();
                }
                if input::is_key_pressed(ctx, Key::Enter) {
                    // todo!("process commands");

                    let mut args = self.command.split_ascii_whitespace();

                    let command = match args.next() {
                        Some(command) => command.to_string(),
                        None => {
                            game::log::warn!("Could not parse command!");
                            self.despawn();
                            return None;
                        },
                    };

                    let args = args.map(std::borrow::ToOwned::to_owned).collect();

                    self.despawn();

                    return Some(
                        CommandResult {
                            command,
                            args,
                        }
                    );

                }
                if let Some(new) = input::get_text_input(ctx) {
                    self.command.push_str(new);
                }
            },
            false => if input::is_key_pressed(ctx, Key::Slash) && game::is_debug() {
                self.spawn();
            },
        }
        None        
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            const Y: f32 = game::util::HEIGHT - 30.0;
            game::graphics::draw_rectangle(ctx, 8.0, Y, game::graphics::text_len(&1, &self.command) + 10.0, 18.0, game::tetra::graphics::Color::BLACK);
            game::graphics::draw_text_left(ctx, &1, "/", &game::text::TextColor::White, 10.0, Y);
            game::graphics::draw_text_left(ctx, &1, &self.command, &game::text::TextColor::White, 16.0, Y);
        }
    }

}

impl Entity for Console {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.reset();
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

impl Reset for Console {
    fn reset(&mut self) {
        self.command.clear();
    }
}