use event::EventWriter;

use crate::engine::{
    controls::{pressed, Control},
    graphics::{Color, CreateDraw, DrawImages, DrawShapes, Graphics, Texture},
    music::{play_music, stop_music, MusicId},
    App, Plugins,
};

use crate::state::{MainStates, StateMessage};

pub struct TitleState {
    accumulator: f32,

    title: Texture,
    trademark: Texture,
    subtitle: Texture,
    charizard: Texture,
    start: Texture,
    copyright: Texture,

    sender: EventWriter<StateMessage>,
}

impl TitleState {
    const MUSIC: MusicId = unsafe {
        MusicId::from_bytes_unchecked([
            0x74, 0x69, 0x74, 0x6C, 0x65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    };

    const TOP: Color = Color::new(248.0 / 255.0, 88.0 / 255.0, 0.0, 1.0);
    const MIDDLE: Color = Color::new(64.0 / 255.0, 176.0 / 255.0, 160.0 / 255.0, 1.0);
    const BOTTOM: Color = Color::new(136.0 / 255.0, 0.0, 0.0, 1.0);

    pub(crate) fn new(
        gfx: &mut Graphics,
        sender: EventWriter<StateMessage>,
    ) -> Result<Self, String> {
        Ok(Self {
            accumulator: 0.0,
            title: gfx
                .create_texture()
                .from_image(include_bytes!("../../../assets/scenes/title/title.png"))
                .build()?,
            trademark: gfx
                .create_texture()
                .from_image(include_bytes!("../../../assets/scenes/title/trademark.png"))
                .build()?,
            subtitle: gfx
                .create_texture()
                .from_image(include_bytes!("../../../assets/scenes/title/subtitle.png"))
                .build()?,
            charizard: gfx
                .create_texture()
                .from_image(include_bytes!("../../../assets/scenes/title/charizard.png"))
                .build()?,
            start: gfx
                .create_texture()
                .from_image(include_bytes!("../../../assets/scenes/title/start.png"))
                .build()?,
            copyright: gfx
                .create_texture()
                .from_image(include_bytes!("../../../assets/scenes/title/copyright.png"))
                .build()?,
            sender,
        })
    }
}

impl TitleState {
    pub fn start(&mut self, app: &mut App, plugins: &mut Plugins) {
        play_music(app, plugins, &Self::MUSIC);
        self.accumulator = 0.0;
    }

    pub fn end(&mut self, app: &mut App, plugins: &mut Plugins) {
        stop_music(app, plugins);
    }

    pub fn update(&mut self, app: &mut App, plugins: &mut Plugins) {
        self.accumulator += app.timer.delta_f32();
        if pressed(app, plugins, Control::A) || app.mouse.left_was_pressed() {
            let seed = (self.accumulator as usize % u8::MAX as usize) as u8;
            self.sender.send(StateMessage::Seed(seed));
            // self.action = Some(MenuStateAction::SeedAndGoto(seed, MenuStates::MainMenu));
            self.sender.send(StateMessage::Goto(MainStates::Menu));
        }
    }

    pub fn draw(&mut self, gfx: &mut Graphics) {
        let mut draw = gfx.create_draw();
        draw.set_size(crate::WIDTH, crate::HEIGHT);
        draw.clear(Color::BLACK);
        let vsides = draw.height() * 9.0 / 160.0;
        draw.rect((0.0, 0.0), (draw.width(), vsides))
            .color(Self::TOP);
        draw.rect(
            (0.0, draw.height() * 30.0 / 160.0),
            (draw.width(), draw.height() * 82.0 / 160.0),
        )
        .color(Self::MIDDLE);
        draw.rect((0.0, draw.height() - vsides), (draw.width(), vsides))
            .color(Self::BOTTOM);

        draw.image(&self.title).position(3.0, 3.0);
        draw.image(&self.trademark).position(158.0, 53.0);
        draw.image(&self.subtitle).position(52.0, 57.0);
        if self.accumulator as u8 % 2 == 1 {
            draw.image(&self.start).position(44.0, 130.0);
        }
        draw.image(&self.charizard).position(129.0, 49.0);
        draw.image(&self.copyright).position(47.0, 152.0);
        gfx.render(&draw);
    }
}
