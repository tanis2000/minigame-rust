    use std::any::Any;

pub type ComponentId = u32;

pub trait GenericStorage<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn push(&mut self, value: T) -> usize;
    fn get(&self, index: usize) -> &T;
    fn len(&self) -> usize;
}

pub trait Component: Any + Sized {
    type Storage: GenericStorage<Self>;
    /*
    fn added(&self) {}
    fn removed(&self) {}
    fn render(&self) {}
    */
}

impl<T> GenericStorage<T> for Vec<T> {
    fn new() -> Self {
        return Vec::new();
    }

    fn push(&mut self, value: T) -> usize {
        Vec::push(self, value);
        self.len() - 1
    }

    fn get(&self, index: usize) -> &T {
        &self[index]
    }

    fn len(&self) -> usize {
        self.len()
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