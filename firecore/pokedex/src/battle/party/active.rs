use serde::{Deserialize, Serialize};

use core::slice::{Iter, IterMut};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PartyActive<A> {
    Single([A; 1]),
    Double([A; 2]),
    Triple([A; 3]),
    Other(Vec<A>),
    None,
}

impl<A> Default for PartyActive<A> {
    fn default() -> Self {
        Self::None
    }
}

impl<A> PartyActive<A> {

    fn arr(&self) -> &[A] {
        match self {
            PartyActive::Single(a) => a as &[A],
            PartyActive::Double(a) => a,
            PartyActive::Triple(a) => a,
            PartyActive::Other(a) => a,
            PartyActive::None => &[],
        }
    }

    fn arr_mut(&mut self) -> &mut [A] {
        match self {
            PartyActive::Single(a) => a as &mut [A],
            PartyActive::Double(a) => a,
            PartyActive::Triple(a) => a,
            PartyActive::Other(a) => a,
            PartyActive::None => &mut [],
        }
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.arr().get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut A> {
        self.arr_mut().get_mut(index)
    }

    pub fn set(&mut self, index: usize, active: A) {
        let a = self.arr_mut().get_mut(index);
        if let Some(a) = a {
            *a = active;
        }
    }

    pub fn iter(&self) -> Iter<'_, A> {
        self.arr().iter()
    }
    
    pub fn iter_mut(&mut self) -> IterMut<'_, A> {
        self.arr_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        match self {
            PartyActive::Single(..) => 1,
            PartyActive::Double(..) => 2,
            PartyActive::Triple(..) => 3,
            PartyActive::Other(a) => a.len(),
            PartyActive::None => 0,
        }
    }

}

impl<A: PartialEq> PartyActive<A> {

    pub fn contains(&self, a: &A) -> bool {
        self.arr().contains(a)
    }

}