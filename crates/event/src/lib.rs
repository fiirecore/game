use std::{cell::RefCell, rc::Rc};

type Inner<T> = Rc<RefCell<Vec<T>>>;

pub fn split<T>() -> (EventWriter<T>, EventReader<T>) {
    let queue: Inner<T> = Default::default();
    (EventWriter(queue.clone()), EventReader(queue))
}

#[derive(Clone)]
pub struct EventWriter<T>(Rc<RefCell<Vec<T>>>);

impl<T> EventWriter<T> {
    pub fn send(&self, t: T) {
        self.0.borrow_mut().push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

pub struct EventReader<T>(Rc<RefCell<Vec<T>>>);

impl<T> EventReader<T> {
    pub fn read(&self) -> Option<T> {
        let mut v = self.0.borrow_mut();
        if !v.is_empty() {
            Some(v.remove(0))
        } else {
            None
        }
    }
}
