use game::{
    deps::tetra::{
        graphics::{Color, Rectangle, Texture},
        math::Vec2,
        Context,
    },
    graphics::{byte_texture, position},
    util::{Reset, WIDTH},
};

use super::TransitionState;

pub struct BattleTrainerPartyIntro {
    state: Option<TransitionState>,
    bar: Texture,
    ball: Texture,
    player: u8,
    opponent: u8,
    counter: u8,
    bar_position: f32,
    ball_position: f32,
}

impl BattleTrainerPartyIntro {
    const BAR_WIDTH: f32 = 104.0;
    const BAR_HIDDEN: f32 = 48.0;
    const RIGHT_BALL_POSITION: f32 = 76.0;
    const OPACITY_LEN: f32 = 128.0;

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            state: None,
            bar: byte_texture(
                ctx,
                include_bytes!("../../assets/gui/bar.png"),
            ),
            ball: byte_texture(
                ctx,
                include_bytes!("../../assets/gui/owned.png"),
            ),
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

    pub fn draw(&self, ctx: &mut Context) {
        if self.state.is_some() {
            let opacity = Color::rgba(
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
                ctx,
                Vec2::new(WIDTH, 96.0),
                true,
                self.player,
                opacity,
                distance,
            );

            // Opponent

            self.draw_gui(
                ctx,
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
        ctx: &mut Context,
        pos: Vec2<f32>,
        invert: bool,
        len: u8,
        opacity: Color,
        distance: u16,
    ) {
        let invert = if invert { -1.0 } else { 1.0 };

        self.bar.draw(
            ctx,
            position(
                pos.x + (self.bar_position - Self::BAR_HIDDEN) * invert,
                pos.y,
            )
            .color(opacity)
            .scale(Vec2::new(invert, 1.0)),
        );

        self.bar.draw_region(
            ctx,
            Rectangle::new(0.0, 0.0, 1.0, 4.0),
            position(
                pos.x + (self.bar_position - Self::BAR_HIDDEN - Self::OPACITY_LEN) * invert,
                pos.y,
            )
            .color(opacity)
            .scale(Vec2::new(invert * Self::OPACITY_LEN, 1.0)),
        );

        for i in 0..self.counter {
            self.ball.draw_region(
                ctx,
                Rectangle::new(0.0, if len > i { 0.0 } else { 7.0 }, 7.0, 7.0),
                position(
                    pos.x
                        + (self.bar_position.max(0.0) + Self::RIGHT_BALL_POSITION
                            - (i as u16 * distance) as f32)
                            * invert,
                    pos.y - 9.0,
                )
                .color(opacity)
                .scale(Vec2::new(invert, 1.0)),
            );
        }

        if self.ball_position != 0.0 && self.counter < 6 {
            self.ball.draw_region(
                ctx,
                Rectangle::new(0.0, if len > self.counter { 0.0 } else { 7.0 }, 7.0, 7.0),
                position(
                    pos.x
                        + invert
                            * (Self::RIGHT_BALL_POSITION + self.ball_position
                                - (self.counter * 10) as f32),
                    pos.y - 9.0,
                )
                .color(opacity)
                .scale(Vec2::new(invert, 1.0)),
            );
        }
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
}

impl Reset for BattleTrainerPartyIntro {
    fn reset(&mut self) {
        self.counter = 0;
        self.bar_position = -Self::BAR_WIDTH;
        self.ball_position = self.bar_position;
    }
}
