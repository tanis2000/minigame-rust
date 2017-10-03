use component::Component;
use log::Log;
use std::vec::Vec;
use std::rc::Rc;
use std::boxed::Box;
use std::any::Any;

pub enum LockMode {
    Open,
    Locked,
    Error,
}

pub struct ComponentList {
    components: Vec<Rc<Box<Any>>>,
    to_add: Vec<Rc<Box<Any>>>,
    to_remove: Vec<Rc<Box<Any>>>,
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
                match self.components.iter().find(|&c| c as *const _ == component as *const _) {
                    None => { 
                        comp = Some(component);
                    }, 
                    Some(_) => {
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
                self.components.iter().position(|c| c as *const _ == component as *const _).map(|e| {
                    comp = Some(component);
                    index = Some(e);
                });
                let c = comp.unwrap();
                let e = index.unwrap();
                c.removed();    
                self.components.remove(e); 
            }
            self.to_remove.clear();
        }
    }

    pub fn get_components(&self) -> &Vec<Rc<Box<Any>>> {
        &self.components
    }

    pub fn add<C: Component>(&mut self, component: C) {
        match self.lock_mode {
            LockMode::Open => {
                let mut comp = None;
                match self.components.iter().find(|&c| &***c as *const _ == &component as *const _) {
                    None => {
                        comp = Some(component);
                    },
                    Some(_) => {
                        Log::warning("Component has already been added before to this same entity");
                    }
                }
                let c = comp.unwrap();
                c.added();
                self.components.push(Rc::new(Box::new(c)));
            },
            LockMode::Locked => {
                let found_comp: bool;
                match self.components.iter().find(|&c| &***c as *const _ == &component as *const _) {
                    None => {
                        found_comp = false;
                    },
                    Some(_) => {
                        found_comp = true;
                    }
                }
                let found_to_add: bool;
                match self.to_add.iter().find(|&c| &***c as *const _ == &component as *const _) {
                    None => {
                        found_to_add = false;
                    },
                    Some(_) => {
                        found_to_add = true;
                    }
                }
                if !found_to_add && !found_comp {
                    self.to_add.push(Rc::new(Box::new(component)));
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

    pub fn render_components(&self) {
        for c in &self.components {
            c.render();
        }
    }

    pub fn update(&self) {
        /*
        self.set_lock_mode(LockMode::Locked);
        for component in self.components {
            if component.get_active() {
                component.update();
            }
        }
        self.set_lock_mode(LockMode::Open);
        */
    }

}