use soloud::WavStream;

pub mod context;
pub mod music;
pub mod sound;

pub(crate) struct ReadableStream(WavStream);

unsafe impl Send for ReadableStream {}
unsafe impl Sync for ReadableStream {}