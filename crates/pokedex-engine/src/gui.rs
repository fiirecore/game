use core::cell::Cell;
use std::ops::Deref;

pub mod bag;
pub mod health;
pub mod party;
pub mod pokemon;

pub const LEVEL_PREFIX: &str = "Lv";

#[derive(Debug, Clone, Copy)]
pub struct SizedStr<const S: usize>([u8; S]);

impl<const S: usize> SizedStr<S> {
    pub fn new(text: impl std::fmt::Display) -> std::io::Result<Self> {
        let mut this = Self([0u8; S]);
        this.replace(text)?;
        Ok(this)
    }

    pub fn replace(&mut self, text: impl std::fmt::Display) -> std::io::Result<()> {
        use std::io::Write;
        write!(&mut self.0 as &mut [u8], "{}", text)
    }
}

impl<const S: usize> Deref for SizedStr<S> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

fn cellref<'a, T>(cell: &'a Cell<T>) -> &'a T {
    unsafe { &*cell.as_ptr() }
}

fn cellmut<'a, T>(cell: &'a Cell<T>) -> &'a mut T {
    unsafe { &mut *cell.as_ptr() }
}
