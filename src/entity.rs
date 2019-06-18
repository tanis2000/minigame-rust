extern crate cgmath;

use component::Component;
use colliderlist::ColliderList;
use std::vec::Vec;
use std::rc::Rc;
use std::any::TypeId;
use self::cgmath::Vector2;

pub type Entity = usize;

pub struct EntityComponent {
    component_type: TypeId,
    component_index: usize,
}

impl EntityComponent {
    pub fn new<C: Component>(index: usize) -> Self {
        EntityComponent {
            component_type: TypeId::of::<C>(),
            component_index: index,
        }
    }
    
    pub fn get_component_type(&self) -> &TypeId {
        return &self.component_type;
    }

    pub fn get_component_index(&self) -> &usize {
        return &self.component_index;
    }
}

pub struct EntityData {
    components: Vec<EntityComponent>,
}

impl EntityData {
    pub fn new() -> Self {
        EntityData {
            components: Vec::new(),
        }
    }

    pub fn get_components(&self) -> &Vec<EntityComponent> {
        return &self.components;
    }

    pub fn get_components_mut(&mut self) -> &mut Vec<EntityComponent> {
        return &mut self.components;
    }

}
/*
pub struct Entity {
    id: u32,    
    pub position: Vector2<f32>,
    active: bool,
    visibile: bool,
    collidable: bool,
    components: ComponentList,
    tags: Vec<u32>,
    colliders: ColliderList,
    actual_depth: f32,
    depth: i32,
}

impl Entity {
    pub fn new() -> Self {
        let e = Entity {
            id: 0,
            position: Vector2::new(0.0, 0.0),
            active: true,
            visibile: true,
            collidable: true,
            components: ComponentList::new(),
            tags: Vec::new(),
            colliders: ColliderList::new(),
            actual_depth: 0.0,
            depth: 0,
        };
        e
    }

    pub fn add(&mut self, component: Rc<Component>) {
        self.components.add(component);
    }

    pub fn remove(&mut self, component: &Component) {
        self.components.remove(component);
    }

    pub fn render(&self) {
        self.components.render_components();
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn entity_added(&self) {
        /*
        this->scene = scene;
        for (auto const &component : components.GetComponents()) {
            component->EntityAdded();
        }
        for (auto const &collider : *colliders.GetColliders()) {
            collider->EntityAdded();
        }
        scene->SetActualDepth(this);
        */
    }

    pub fn entity_removed(&self) {
        /*
        for (auto const &component : components.GetComponents()) {
            component->EntityRemoved();
        }
        for (auto const &collider : *colliders.GetColliders()) {
            collider->EntityRemoved();
        }
        scene = NULL;
        */
    }

    pub fn awake(&self) {
        
    }

    pub fn scene_begin(&self) {

    }

    pub fn scene_end(&self) {

    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn update(&self) {
        self.components.update();
        //self.colliders.update();
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

*/