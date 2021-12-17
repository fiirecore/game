use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[deprecated]
pub static LOADING_FINISHED: AtomicBool = AtomicBool::new(false);

#[deprecated]
pub fn finished_loading() {
    LOADING_FINISHED.store(true, Relaxed);
}
