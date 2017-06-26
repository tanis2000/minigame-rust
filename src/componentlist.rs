use component::Component;
use entity::Entity;
use log::Log;
use std::vec::Vec;
use std::rc::Rc;
use std::boxed::Box;
use std::option::Option;
use std::cell::Cell;
use std::collections::HashMap;

pub enum LockMode {
    Open,
    Locked,
    Error,
}

pub struct ComponentList {
    components: Vec<Rc<Component>>,
    to_add: Vec<Rc<Component>>,
    to_remove: Vec<Rc<Component>>,
    lock_mode: LockMode,
}

impl ComponentList {
    pub fn new() -> Self {
        ComponentList {
            components: Vec::new(),
            to_add: Vec::new(),
            to_remove: Vec::new(),
            lock_mode: LockMode::Open,
        }
    }

    pub fn get_lock_mode(self) -> LockMode {
        self.lock_mode
    }

    pub fn set_lock_mode(mut self, lock_mode: LockMode) {
        self.lock_mode = lock_mode;
        if self.to_add.len() > 0 {
            for component in &self.to_add {
                let mut comp = None;
                match self.components.iter().find(|&c| *c == *component) {
                    None => { 
                        comp = Some(component);
                    }, 
                    Some(v) => {
                        Log::warning("Component already exists");
                    }
                }
                let mut c = comp.unwrap();
                c.added();
                self.components.push(c.clone()); 
            }
            self.to_add.clear();
        }

        if self.to_remove.len() > 0 {
            for component in &self.to_remove {
                let mut comp = None;
                let mut index = None;
                self.components.iter().position(|c| *c == *component).map(|e| {
                    comp = Some(component);
                    index = Some(e);
                });
                let mut c = comp.unwrap();
                let mut e = index.unwrap();
                c.removed();    
                self.components.remove(e); 
            }
            self.to_remove.clear();
        }
    }

    pub fn get_components(&self) -> &Vec<Rc<Component>> {
        &self.components
    }

    pub fn add(&mut self, component: Component) {
        match self.lock_mode {
            LockMode::Open => {
                let mut comp = None;
                match self.components.iter().find(|&c| **c == component) {
                    None => {
                        comp = Some(component);
                    },
                    Some(v) => {
                        Log::warning("Component has already been added before to this same entity");
                    }
                }
                let mut c = comp.unwrap();
                c.added();
                self.components.push(Rc::new(c));
            },
            LockMode::Locked => {
                let mut found_comp = false;
                match self.components.iter().find(|&c| **c == component) {
                    None => {
                        found_comp = false;
                    },
                    Some(v) => {
                        found_comp = true;
                    }
                }
                let mut found_to_add = false;
                match self.to_add.iter().find(|&c| **c == component) {
                    None => {
                        found_to_add = false;
                    },
                    Some(v) => {
                        found_to_add = true;
                    }
                }
                if !found_to_add && !found_comp {
                    self.to_add.push(Rc::new(component));
                } else {
                    Log::warning("Component has already been added before to this same entity");
                }
            },
            LockMode::Error => {
                Log::error("Cannot add or remove components at this time!");
                panic!("Cannot add or remove components at this time!");
            }
        }
    }

    pub fn remove(&self, component: &Component) {
    // TODO
    }

}