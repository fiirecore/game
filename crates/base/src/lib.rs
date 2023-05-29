mod audio;

pub extern crate firecore_text as text;
pub extern crate notan;

pub use hashbrown::{HashMap, HashSet, hash_map::DefaultHashBuilder as RandomState};

pub use notan::{math, AppState};

pub mod music {
    pub use super::audio::{add_music, get_current_music, play_music, stop_music, MusicId};
}

pub mod sound {
    pub use super::audio::{add_sound, play_sound, SoundId, SoundVariant};
}

pub mod controls;
pub mod graphics;
pub mod gui;

pub use context::*;

pub use notan::{egui, log};

pub use notan::prelude::{App, Plugins};

mod context {
    use notan::prelude::Plugins;

    use crate::controls::context::ControlsContext;

    pub fn setup(plugins: &mut Plugins) {
        plugins.add(ControlsContext::default());
        #[cfg(feature = "audio")]
        plugins.add(crate::audio::backend::AudioContext::default());
    }

    #[cfg(feature = "audio")]
    pub use crate::audio::backend::AudioContext;

    // pub struct EngineContext {
    //     pub(crate) controls: ControlsContext,
    //     // pub(crate) text: TextRenderer,
    //     // pub(crate) panel: Texture,
    //     #[cfg(feature = "audio")]
    //     pub(crate) audio: crate::audio::backend::AudioContext,
    // }

    // impl UserContext for EngineContext {
    //     fn new(ctx: &mut Context) -> Result<Self, EngineError> {
    //         Ok(Self {
    //             // text: TextRenderer::new(ctx)?,
    //             controls: ControlsContext::default(),
    //             panel: gfx.create_texture().from_image(include_bytes!("../assets/panel.png"))?,
    //             #[cfg(feature = "audio")]
    //             audio: Default::default(),
    //         })
    //     }
    // }
}
