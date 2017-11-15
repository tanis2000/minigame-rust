use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use entity::IdNumber;
use log::Log;

pub struct InstanceData {
    names: Vec<String>,
}

impl InstanceData {
    pub fn new() -> Self {
        InstanceData {
            names: Vec::new(),
        }
    }
}

pub struct Instance {
    pub i: usize,
}

impl Instance {
    pub fn new() -> Self {
        Instance {
            i: 0,
        }
    }
}

pub struct DebugNameComponentManager {
    data: InstanceData,
    map: HashMap<IdNumber, usize>, 
}

impl DebugNameComponentManager {
    pub fn new() -> Self {
        DebugNameComponentManager {
            data: InstanceData::new(),
            map: HashMap::new(),
        }
    }

    pub fn make_instance(&self, i: usize) -> Instance {
        let mut inst = Instance::new();
        inst.i = i;
        inst
    }

    pub fn lookup(&self, entity_id: IdNumber) -> Instance {
        let i: usize;
        match self.map.get(&entity_id) {
            Some(idx) => {
                i = *idx;
            },
            None => {
                i = 0xffffffff;
            }
        }
        self.make_instance(i)
    }

    pub fn create(&mut self, entity_id: IdNumber) -> Instance {
        self.data.names.push(String::from("empty"));
        self.map.entry(entity_id).or_insert(self.data.names.len()-1);
        self.make_instance(self.data.names.len()-1)
    }

    pub fn get_name(&self, i: Instance) -> &String {
        &self.data.names[i.i]
    }

    pub fn set_name(&mut self, i: Instance, name: String) {
        self.data.names[i.i] = name;
    }

    pub fn update(&self, dt: f32) {
        for name in &self.data.names {
            Log::debug(&name);
        }
    }
}
