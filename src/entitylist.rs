use scene::Scene;
use entity::Entity;
use entity::IdNumber;
use std::vec::Vec;
use std::rc::Rc;

pub struct EntityList {
    next_id: IdNumber,
    entities: Vec<Rc<Entity>>,
    to_add: Vec<Rc<Entity>>,
    to_awake: Vec<Rc<Entity>>,
    to_remove: Vec<Rc<Entity>>,
    unsorted: bool,
}

impl EntityList {
    pub fn new() -> Self {
        EntityList {
            next_id: 0,
            entities: Vec::new(),
            to_add: Vec::new(),
            to_awake: Vec::new(),
            to_remove: Vec::new(),
            unsorted: false,
        }
    }

    pub fn create_entity(&mut self) -> Rc<Entity> {
        let e = Entity::new();
        self.next_id += 1;
        let rce = Rc::new(e);
        self.add(rce.clone());
        rce
    }

    pub fn add(&mut self, entity: Rc<Entity>) {
        let to_add_contains = self.to_add.iter().any(|e| **e == *entity);
        let entities_contains = self.entities.iter().any(|e| **e == *entity);
        if !to_add_contains && !entities_contains {
            self.to_add.push(entity);
        }
    }
}