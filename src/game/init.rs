use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[deprecated]
pub static LOADING_FINISHED: AtomicBool = AtomicBool::new(false);

pub fn logger() {
    use log::LevelFilter;
    use simple_logger::SimpleLogger;

    // Initialize logger

    let logger = SimpleLogger::new();

    #[cfg(debug_assertions)]
    let logger = logger.with_level(LevelFilter::Trace);
    #[cfg(not(debug_assertions))]
    let logger = logger.with_level(LevelFilter::Info);

    logger
        .init()
        .unwrap_or_else(|err| panic!("Could not initialize logger with error {}", err));
}

pub fn finished_loading() {
    LOADING_FINISHED.store(true, Relaxed);
}
