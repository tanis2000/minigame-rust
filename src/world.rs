use component::{Component, ComponentId, GenericStorage};
use entity::{Entity, EntityComponent, EntityData};
use bit_set::BitSet;
use std::boxed::Box;
use std::rc::Rc;
use std::vec::Vec;
use std::collections::HashMap;
use std::any::{Any, TypeId};

pub struct World {
    next_entity_id: Entity,
    entities: Vec<EntityData>,
    active_entities: BitSet,
    components: HashMap<TypeId, Box<dyn Any>>,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            entities: Vec::new(),
            active_entities: BitSet::new(),
            components: HashMap::new(),
            systems: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let e = self.next_entity_id;
        self.entities.push(EntityData::new());
        self.active_entities.insert(e);
        self.next_entity_id += 1;
        return e;
    }

    pub fn register_component_with_storage<C: Component>(&mut self) {
        self.components.insert(TypeId::of::<C>(), Box::new(C::Storage::new()));
    }
    
    pub fn add_component_to_storage<C: Component>(&mut self, entity: Entity, component: C) -> usize {
        let storage = self.components.get_mut(&TypeId::of::<C>()).unwrap().downcast_mut::<C::Storage>().unwrap();
        storage.insert(entity, component);
        //return storage.len() - 1;
        return entity;
    }
    
    fn get_component<C: Component>(&self, index: usize) -> &C {
        let storage = self.components[&TypeId::of::<C>()]
            .downcast_ref::<C::Storage>()
            .unwrap();
        let component = storage.get(index);
        component.unwrap()
    }

    pub fn add_component_to_entity<C: Component>(&mut self, entity: Entity, component: C) {
        let index = self.add_component_to_storage(entity, component);
        let entity_data = self.entities.get_mut(entity).unwrap();
        let ec = EntityComponent::new::<C>(index);
        entity_data.get_components_mut().push(ec);
        for system in &mut self.systems {
            for component in system.get_components() {
                if component == TypeId::of::<C>() {
                    println!("calling add_entity for {}", entity);
                    system.add_entity(entity);
                }
            }
        }
    }
    
    pub fn get_component_for_entity<C: Component>(&self, entity: Entity) -> Option<&C> {
        let entity_data = &self.entities.get(entity);
        for entity_component in entity_data.unwrap().get_components() {
            if entity_component.get_component_type() == &TypeId::of::<C>() {
                let index = entity_component.get_component_index();
                return Some(self.get_component::<C>(*index));
            }
        }
        return None;
    }

    pub fn get_component_index_for_entity<C: Component>(&self, entity: Entity) -> Option<usize> {
        let entity_data = &self.entities.get(entity);
        for entity_component in entity_data.unwrap().get_components() {
            if entity_component.get_component_type() == &TypeId::of::<C>() {
                return Some(*entity_component.get_component_index());
            }
        }
        return None;
    }

    pub fn get_components_of_type<C: Component>(&self) -> &HashMap<usize, C> {
        let storage = self.components[&TypeId::of::<C>()]
            .downcast_ref::<C::Storage>()
            .unwrap();
        return storage.all();
    }

    pub fn remove_component_from_storage<C: Component>(&mut self, component_index: usize) -> C {
        let storage = self.components.get_mut(&TypeId::of::<C>()).unwrap().downcast_mut::<C::Storage>().unwrap();
        return storage.remove(component_index).unwrap();
    }

    pub fn remove_component_from_entity<C: Component>(&mut self, entity: Entity) {
        let index = self.get_component_index_for_entity::<C>(entity);
        match index {
            Some(index) => {
                self.remove_component_from_storage::<C>(index);
                self.entities.remove(entity);
            },
            None => {}
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        for storage in &self.components {
        }
    }

    pub fn add_system<S: System>(&mut self, system: S) -> usize {
        self.systems.push(Box::new(system));
        return self.systems.len() - 1;
    }

    pub fn process(&self, dt: f32, user_data: &SystemData) {
        for system in &self.systems {
            for entity in system.get_entities() {
                println!("calling process");
                system.process(entity, dt, user_data);
            }
        }
    }
}

pub trait SystemData {
    fn get_context(&self) -> Any;
}

pub trait System: Any {
    fn get_entities(&self) -> Vec<Entity>;
    fn process(&self, entity: Entity, dt: f32, user_data: &SystemData);
    fn get_components(&self) -> Vec<TypeId>;
    fn add_entity(&mut self, entity: Entity);
}

pub struct BaseSystem {
    watch: BitSet,
    entities: Vec<Entity>,
    components: Vec<TypeId>,
}

impl BaseSystem {
    pub fn new() -> Self {
        BaseSystem {
            watch: BitSet::new(),
            entities: Vec::new(),
            components: Vec::new(),
        }
    }

    /*
    pub fn watch_component(&mut self, component_id: usize) {
        self.watch.insert(component_id);
    }
    */

    pub fn watch_component<C: Component>(&mut self) {
        self.components.push(TypeId::of::<C>());
    }

    pub fn process(&self, entity: Entity, dt: f32) {
        println!("BaseSystem process() called");
    }

    pub fn get_components(&self) -> Vec<TypeId> {
        return self.components.clone(); // TODO: this is very bad for performance
    }

    pub fn get_entities(&self) -> Vec<Entity> {
        return self.entities.clone(); // TODO: this is very bad for performance
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

/*
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
*/