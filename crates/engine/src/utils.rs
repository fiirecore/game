pub use fiirengine::utils::*;

/// deprecated
pub const WIDTH: f32 = 240.0;
/// deprecated
pub const HEIGHT: f32 = 160.0;

pub trait Entity {
    fn spawn(&mut self);

    fn despawn(&mut self);

    fn alive(&self) -> bool;
}

pub trait Reset {
    fn reset(&mut self);
}

pub trait Completable: Reset {
    fn finished(&self) -> bool;
}

pub fn type_name<T: ?Sized>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}
