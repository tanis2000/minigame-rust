use entity::Entity;
use entity::IdNumber;
use std::vec::Vec;
use std::rc::Rc;

pub struct EntityList {
    next_id: IdNumber,
    entities: Vec<Entity>,
    to_add: Vec<Entity>,
    to_awake: Vec<Entity>,
    to_remove: Vec<Entity>,
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

    pub fn create_entity(&mut self) -> u32 {
        let e = Entity::new();
        self.next_id += 1;
        let id = e.get_id();
        self.add(e);
        id
    }

    pub fn add(&mut self, entity: Entity) {
        let to_add_contains = self.to_add.iter().any(|e| *e == entity);
        let entities_contains = self.entities.iter().any(|e| *e == entity);
        if !to_add_contains && !entities_contains {
            self.to_add.push(entity);
        }
    }

    pub fn update_lists(&mut self) {
        if self.to_add.len() > 0 {
            for to_add_it in self.to_add.drain(..) {
                let entities_contains = self.entities.iter().any(|e| *e == to_add_it);
                if !entities_contains {
                    to_add_it.entity_added();
                    to_add_it.awake();
                    self.entities.push(to_add_it);
                    //scene.tag_lists.entity_added(to_add_it);
                }
            }
            self.unsorted = true;
        }

        if self.to_remove.len() > 0 {
            for to_remove_it in self.to_remove.drain(..) {
                let entities_contains = self.entities.iter().any(|e| *e == to_remove_it);
                if entities_contains {
                    to_remove_it.entity_removed();
                    let entities_position = self.entities.iter().position(|e| *e == to_remove_it);
                    self.entities.remove(entities_position.unwrap());
                    //scene.tag_lists.entity_removed(to_remove_it);
                }
            }
            self.to_remove.clear();
        }

        if self.unsorted {
            self.unsorted = false;
            //self.entities.sort(entitylist::CompareDepth);
        }

    }

    pub fn render_entities(&self) {
        for e in &self.entities {
            //if (e.visible) {
                e.render();
            //}
        }
    }

    pub fn get_entity_mut(&mut self, entity_id: u32) -> Option<&mut Entity> {
        let to_add_contains = self.to_add.iter().any(|e| e.get_id() == entity_id);
        if to_add_contains {
            return self.to_add.iter_mut().find(|e| e.get_id() == entity_id);
        }
        let entities_contains = self.entities.iter().any(|e| e.get_id() == entity_id);
        if entities_contains {
            return self.entities.iter_mut().find(|e| e.get_id() == entity_id);
        }
        return None;
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn update(&self) {
        for entity in &self.entities {
            if entity.get_active() {
                entity.update();
            }
        }
    }
}