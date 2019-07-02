use std::any::Any;
use std::collections::HashMap;

pub type ComponentId = usize;

pub trait GenericStorage<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn insert(&mut self, index: usize, value: T) -> usize;
    fn get(&self, index: usize) -> Option<&T>;
    fn len(&self) -> usize;
    fn all(&self) -> &HashMap<usize, T>;
    fn remove(&mut self, index: usize) -> Option<T>;
}

pub trait Component: Any + Sized {
    type Storage: GenericStorage<Self>;
    /*
    fn added(&self) {}
    fn removed(&self) {}
    fn render(&self) {}
    */
}

impl<T> GenericStorage<T> for HashMap<usize, T> {
    fn new() -> Self {
        return HashMap::new();
    }

    fn insert(&mut self, index: usize, value: T) -> usize {
        self.insert(index, value);
        self.len() - 1
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.get(&index)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn all(&self) -> &HashMap<usize, T> {
        self
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        self.remove(&index)
    }
}


/*impl<T: Any> Component for T {

}*/

/*
type IdNumber = u32;

pub struct Component {
    id: IdNumber,
    visible: bool,
    active: bool,
}

impl Component {
    pub fn new() -> Self {
        let c = Component {
            id: 0,
            visible: true,
            active: true,
        };
        c
    }

    pub fn added(&self) {

    }

    pub fn removed(&self) {

    }

    pub fn entity_added() {

    }

    pub fn entity_removed() {

    }

    pub fn update(&self) {

    }

    pub fn render(&self) {

    }

    pub fn debug_render(&self) {

    }

}

struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    friction: f32,
    dx: f32,
    dy: f32,
}

impl PartialEq for Component {
    fn eq(&self, other: &Component) -> bool {
        self.id == other.id
    }
}
*/