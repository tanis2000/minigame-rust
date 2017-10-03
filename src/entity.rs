extern crate cgmath;

use component::Component;
use componentlist::ComponentList;
use collider::Collider;
use colliderlist::ColliderList;
use std::vec::Vec;
use std::rc::Rc;
use self::cgmath::Vector2;

pub type IdNumber = u32;

pub trait EntityTrait {
    /*
    fn added(scene: &Scene);
    fn removed(scene: &Scene);
    fn awake(scene: &Scene);
    fn scene_begin();
    fn scene_end();
    fn update();
    fn render();
    fn debug_render();
    fn add(&mut self, component: Component);
    fn remove(&mut self, component: &Component);
    fn get() -> Rc<Component>;
    fn add_collider(collider: &Collider);
    fn remove_collider(collider: &Collider);
    fn tag(tag: u32);
    fn untag(tag: u32);
    fn collide_check(other: &Entity);
    fn set_depth(depth: i32);
    fn get_depth() -> i32;
    */
}

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

