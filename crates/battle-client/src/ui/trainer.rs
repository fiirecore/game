use pokengine::engine::{
    graphics::{Color, Texture},
    math::Vec2,
    notan::draw::{Draw, DrawImages, DrawTransform},
};

use crate::InitBattleGuiTextures;

#[derive(Debug)]
enum TransitionState {
    Begin, // runs on spawn methods
    Run,
    End, // spawns next state and goes back to beginning
}

impl Default for TransitionState {
    fn default() -> Self {
        Self::Begin
    }
}

pub struct PokemonCount {
    state: Option<TransitionState>,
    bar: Texture,
    ball: Texture,
    player: u8,
    opponent: u8,
    counter: u8,
    bar_position: f32,
    ball_position: f32,
}

impl PokemonCount {
    const BAR_WIDTH: f32 = 104.0;
    const BAR_HIDDEN: f32 = 48.0;
    const RIGHT_BALL_POSITION: f32 = 76.0;
    const OPACITY_LEN: f32 = 128.0;

    pub fn new(btl: &InitBattleGuiTextures) -> Self {
        Self {
            state: None,
            bar: btl.bar.clone(),
            ball: btl.pokeball.clone(),
            player: 0,
            opponent: 0,
            counter: 0,
            bar_position: 0.0,
            ball_position: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if let Some(state) = &self.state {
            match state {
                TransitionState::Begin => match self.bar_position < 0.0 {
                    true => {
                        self.bar_position += 240.0 * delta;
                        if self.bar_position > 0.0 {
                            self.bar_position = 0.0;
                        }
                    }
                    false => match self.ball_position < 0.0 {
                        true => {
                            self.ball_position += 480.0 * delta;
                            if self.ball_position > 0.0 {
                                self.ball_position = 0.0;
                            }
                        }
                        false => {
                            match self.counter == 6 {
                                true => self.state = Some(TransitionState::Run),
                                false => self.counter += 1,
                            }
                            self.ball_position = -Self::RIGHT_BALL_POSITION;
                        }
                    },
                },
                TransitionState::Run => (),
                TransitionState::End => {
                    self.bar_position += delta * 240.0;
                }
            }
        }
    }

    pub fn draw(&self, draw: &mut Draw) {
        if self.state.is_some() {
            let opacity = Color::new(
                1.0,
                1.0,
                1.0,
                if matches!(self.state, Some(TransitionState::End)) {
                    (1.0 - self.bar_position / Self::OPACITY_LEN).max(0.0)
                } else {
                    1.0
                },
            );

            let distance = if matches!(self.state, Some(TransitionState::End)) {
                10 + (self.bar_position / 6.0) as u16
            } else {
                10
            };

            // Player

            self.draw_gui(
                draw,
                Vec2::new(draw.width(), 96.0),
                true,
                self.player,
                opacity,
                distance,
            );

            // Opponent

            self.draw_gui(
                draw,
                Vec2::new(0.0, 41.0),
                false,
                self.opponent,
                opacity,
                distance,
            );
        }
    }

    pub fn draw_gui(
        &self,
        draw: &mut Draw,
        pos: Vec2,
        invert: bool,
        len: u8,
        opacity: Color,
        distance: u16,
    ) {
        let invert_f = if invert { -1.0f32 } else { 1.0 };

        let invert_sub = if invert { self.bar.width() } else { 0.0 };

        {
            let (x, sx) = match invert {
                true => (pos.x + self.bar.width(), -1.0),
                false => (pos.x, 1.0),
            };

            draw.image(&self.bar)
                .position(
                    x + (self.bar_position - Self::BAR_HIDDEN) * invert_f - invert_sub,
                    pos.y,
                )
                .color(opacity)
                .scale(sx, 1.0);
        }

        // self.bar.draw(
        //     draw,
        //     pos.x + (self.bar_position - Self::BAR_HIDDEN - Self::OPACITY_LEN) * invert_f
        //         - invert_sub,
        //     pos.y,
        //     DrawParams {
        //         color: opacity,
        //         source: Some(Rectangle::new(0.0, 0.0, 1.0, 4.0)),
        //         dest_size: Some(vec2(Self::OPACITY_LEN, self.bar.height())),
        //         flip_x: invert,
        //         ..Default::default()
        //     },
        // );

        // for i in 0..self.counter {
        //     self.ball.draw(
        //         draw,
        //         pos.x
        //             + (self.bar_position.max(0.0) + Self::RIGHT_BALL_POSITION
        //                 - (i as u16 * distance) as f32)
        //                 * invert_f,
        //         pos.y - 9.0,
        //         DrawParams {
        //             source: Some(Rectangle::new(
        //                 0.0,
        //                 if len > i { 0.0 } else { 7.0 },
        //                 7.0,
        //                 7.0,
        //             )),
        //             color: opacity,
        //             flip_x: invert,
        //             ..Default::default()
        //         },
        //     );
        // }

        // if self.ball_position != 0.0 && self.counter < 6 {
        //     self.ball.draw(
        //         draw,
        //         pos.x
        //             + invert_f
        //                 * (Self::RIGHT_BALL_POSITION + self.ball_position
        //                     - (self.counter * 10) as f32),
        //         pos.y - 9.0,
        //         DrawParams {
        //             source: Some(Rectangle::new(
        //                 0.0,
        //                 if len > self.counter { 0.0 } else { 7.0 },
        //                 7.0,
        //                 7.0,
        //             )),
        //             color: opacity,
        //             flip_x: invert,
        //             ..Default::default()
        //         },
        //     );
        // }
    }

    pub fn end(&mut self) {
        self.state = Some(TransitionState::End);
    }

    pub fn ending(&self) -> bool {
        matches!(self.state, Some(TransitionState::End))
    }

    pub fn spawn(&mut self, player: usize, opponent: usize) {
        self.player = player as u8;
        self.opponent = opponent as u8;
        self.state = Some(TransitionState::Begin);
        self.reset();
    }

    pub fn despawn(&mut self) {
        self.state = None;
    }

    pub fn reset(&mut self) {
        self.counter = 0;
        self.bar_position = -Self::BAR_WIDTH;
        self.ball_position = self.bar_position;
    }
}
