use pokengine::engine::{graphics::{Texture, Draw, DrawExt, DrawParams, Color}, math::{Vec2, Rect}, App};

#[derive(Default, Clone)]
pub struct Faint {
    pub remaining: f32,
}

impl Faint {
    const DEFAULT_TEXTURE_SIZE: f32 = 64.0;

    pub fn new(texture: Option<&Texture>) -> Self {
        Self {
            remaining: texture
                .as_ref()
                .map(|t| t.height())
                .unwrap_or(Self::DEFAULT_TEXTURE_SIZE),
        }
    }

    pub fn update(&mut self, app: &mut App) -> bool {
        self.remaining -= app.timer.delta_f32() * 128.0;
        self.remaining <= 0.0
    }

    pub fn draw(&self, draw: &mut Draw, texture: &Texture, pos: Vec2, color: Color) {
        if self.remaining > 0.0 {
            draw.texture(
                texture,
                pos.x,
                pos.y - self.remaining,
                DrawParams {
                    source: Some(Rect {
                        x: 0.0,
                        y: 0.0,
                        width: texture.width(),
                        height: self.remaining,
                    }),
                    color,
                    ..Default::default()
                },
            );
        }
    }
}
