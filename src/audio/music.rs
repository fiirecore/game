use macroquad::prelude::info;

impl Music {

    pub fn bind_music(context: &mut crate::util::context::GameContext) {
        info!("Loading music...");
        let time = instant::Instant::now();
        for music in Music::into_enum_iter() {
            context.load_music(music);
        }
        info!("Finished loading world music in {} ms.", time.elapsed().as_millis());
    }

}