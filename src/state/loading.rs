use crate::engine::{
    controls::{pressed, Control},
    graphics::{Color, CreateDraw, Draw, DrawImages, DrawShapes, DrawTransform, Graphics, Texture},
    music::{play_music, MusicId},
    App, Plugins,
};

pub enum LoadingScenes {
    Copyright,
    Gamefreak,
    Pokemon,
    WaitLoading,
}

pub struct LoadingStateManager {
    current: LoadingScenes,

    accumulator: f32,

    version: String,

    copyright: Texture,
    rect_size: f32,
    star: Texture,
    logo: Texture,
    text: Texture,
    loading: Texture,

}

impl LoadingStateManager {
    const MUSIC: MusicId = unsafe {
        MusicId::from_bytes_unchecked([
            0x67, 0x61, 0x6D, 0x65, 0x66, 0x72, 0x65, 0x61, 0x6B, 0, 0, 0, 0, 0, 0, 0,
        ])
    };

    const BACKGROUND: Color = Color::new(24.0 / 255.0, 40.0 / 255.0, 72.0 / 255.0, 1.0);

    pub(crate) fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            current: LoadingScenes::default(),
            accumulator: 0.0,
            version: format!("v{}", crate::VERSION),
            copyright: gfx
                .create_texture()
                .from_image(include_bytes!("../../assets/scenes/loading/copyright.png"))
                .build()?,
            // pokemon: PokemonLoadingScene::new(ctx)?,
            rect_size: 0.0,
            star: gfx
                .create_texture()
                .from_image(include_bytes!("../../assets/scenes/loading/star.png"))
                .build()?,
            logo: gfx
                .create_texture()
                .from_image(include_bytes!("../../assets/scenes/loading/logo.png"))
                .build()?,
            text: gfx
                .create_texture()
                .from_image(include_bytes!("../../assets/scenes/loading/text.png"))
                .build()?,
            loading: gfx
                .create_texture()
                .from_image(include_bytes!("../../assets/scenes/loading/loading.png"))
                .build()?,
        })
    }

    fn queue(&mut self, app: &mut App, plugins: &mut Plugins, state: LoadingScenes) {
        match state {
            LoadingScenes::Copyright => {
                self.accumulator = 0.0;
            }
            LoadingScenes::Gamefreak => {
                self.accumulator = 0.0;
                self.rect_size = 0.0;
                play_music(app, plugins, &Self::MUSIC);
            }
            LoadingScenes::Pokemon => todo!(),
            LoadingScenes::WaitLoading => {
                self.accumulator = 0.0;
            }
        }
        self.current = state;
    }

    pub fn update(&mut self, app: &mut App, plugins: &mut Plugins, delta: f32, loaded: bool) -> bool {
        match self.current {
            LoadingScenes::Copyright => {
                if self.accumulator > 2.0 {
                    if loaded {
                        if self.accumulator > 4.0 {
                            self.queue(app, plugins, LoadingScenes::Gamefreak);
                        }
                        self.accumulator += delta;
                    }
                } else {
                    self.accumulator += delta;
                }
            }
            LoadingScenes::Gamefreak => {
                self.accumulator += delta;
                if self.rect_size < 96.0 {
                    self.rect_size += delta * 480.0;
                    if self.rect_size > 96.0 {
                        self.rect_size = 96.0;
                    }
                }
                if self.accumulator > 8.5 {
                    self.queue(app, plugins, LoadingScenes::Copyright);
                    return true;
                }
            }
            LoadingScenes::Pokemon => {
                self.queue(app, plugins, LoadingScenes::Copyright);
                // self.finish();
            }
            LoadingScenes::WaitLoading => {
                if self.accumulator > 3.0 {
                    self.accumulator -= 3.0;
                }
            }
        }

        pressed(app, plugins, Control::A) && loaded
    }

    pub fn draw(&self, gfx: &mut Graphics, progress: Option<f32>) {
        let mut draw = gfx.create_draw();
        draw.set_size(crate::WIDTH, crate::HEIGHT);
        draw.clear(Color::BLACK);
        match self.current {
            LoadingScenes::Copyright => {
                fade_in_out(
                    &mut draw,
                    &self.copyright,
                    0.0,
                    0.0,
                    self.accumulator,
                    3.0,
                    0.5,
                );
                if let Some(loaded) = progress {
                    draw.rect(
                        (draw.width() / 3.0, draw.height() * 2.0 / 3.0),
                        (loaded * draw.width() / 3.0, (draw.height() / 10.0).ceil()),
                    )
                    .color(Color::WHITE);
                }
            }
            LoadingScenes::Gamefreak => {
                draw.rect(
                    (0.0, (draw.height() - self.rect_size) / 2.0),
                    (240.0, self.rect_size),
                )
                .color(Self::BACKGROUND);

                if self.accumulator < 2.0 {
                    draw.image(&self.star)
                        .position(
                            crate::WIDTH - self.accumulator * 240.0,
                            64.0 + self.accumulator * 5.0,
                        )
                        .rotate(self.accumulator * 2.0);
                }

                fade_in(
                    &mut draw,
                    &self.logo,
                    108.0,
                    45.0,
                    self.accumulator - 6.0,
                    1.0,
                ); //108x, 12y
                fade_in(
                    &mut draw,
                    &self.text,
                    51.0,
                    74.0,
                    self.accumulator - 4.0,
                    1.0,
                ); //51x, 41y
            }
            LoadingScenes::Pokemon => todo!(),
            LoadingScenes::WaitLoading => {
                const WHITE: Color = Color::new(216.0 / 255.0, 216.0 / 255.0, 216.0 / 255.0, 1.0);

                draw.image(&self.loading).position(6.0, 149.0);
                draw.rect((63.0, 152.0), (2.0, 2.0)).color(WHITE);
                if self.accumulator > 1.0 {
                    draw.rect((68.0, 152.0), (2.0, 2.0)).color(WHITE);
                }
                if self.accumulator > 2.0 {
                    draw.rect((73.0, 152.0), (2.0, 2.0)).color(WHITE);
                }
            }
        }
        gfx.render(&draw);
    }
}

impl Default for LoadingScenes {
    fn default() -> Self {
        Self::Copyright
    }
}

pub fn fade_in_out(
    draw: &mut Draw,
    texture: &Texture,
    x: f32,
    y: f32,
    accumulator: f32,
    end_time: f32,
    fade_time: f32,
) {
    if accumulator < fade_time {
        draw.image(texture)
            .position(x, y)
            .alpha(accumulator / fade_time);
    } else if accumulator < end_time - fade_time {
        draw.image(texture).position(x, y);
    } else if accumulator < end_time {
        draw.image(texture)
            .position(x, y)
            .alpha((end_time - accumulator) / fade_time);
    }
}

pub fn fade_in(
    draw: &mut Draw,
    texture: &Texture,
    x: f32,
    y: f32,
    accumulator: f32,
    fade_time: f32,
) {
    draw.image(texture)
        .position(x, y)
        .alpha(accumulator / fade_time);
}
