// pub use fiirengine::utils::*;
pub use hashbrown::{hash_map::DefaultHashBuilder, HashMap, HashSet};

pub trait Entity {
    fn spawn(&mut self);

    fn despawn(&mut self);

    fn alive(&self) -> bool;
}

pub fn type_name<T: ?Sized>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}
