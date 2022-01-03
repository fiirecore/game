pub use fiirengine::*;

mod audio;

pub mod music {
    pub use super::audio::{add_music, get_current_music, play_music, stop_music, MusicId};
}

pub mod sound {
    pub use super::audio::{add_sound, play_sound, SoundId, SoundVariant};
}

pub mod controls;
pub mod graphics;
pub mod gui;
pub mod text;
pub mod utils;

pub use context::*;

mod context {
    use fiirengine::{graphics::Texture, Context, EngineError, UserContext};

    use crate::{controls::ControlsContext, graphics::renderer::TextRenderer};

    pub struct EngineContext {
        pub(crate) controls: ControlsContext,
        pub(crate) text: TextRenderer,
        pub(crate) panel: Texture,
        #[cfg(feature = "audio")]
        pub(crate) audio: crate::audio::backend::AudioContext,
    }

    impl UserContext for EngineContext {
        fn new(ctx: &mut Context) -> Result<Self, EngineError> {
            Ok(Self {
                text: TextRenderer::new(ctx)?,
                controls: ControlsContext::default(),
                panel: Texture::new(ctx, include_bytes!("../assets/panel.png"))?,
                #[cfg(feature = "audio")]
                audio: Default::default(),
            })
        }
    }
}
