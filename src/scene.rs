extern crate cgmath;

use entity::Entity;
use world::{System, SystemData, World};
use collider::Collider;
use component::{Component, ComponentId};
use rectangle::Rectangle;
use renderer::Renderer;
use std::vec::Vec;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use self::cgmath::Vector2;

pub trait SceneTrait {
    fn before_update();
    fn update();
    fn after_update();
    fn begin();
    fn end();
    fn before_render();
    fn render();
    fn after_render();
    fn set_actual_depth(entity: Rc<Entity>);
    fn get_entities_with_tag(tag: u32) -> Vec<Rc<Entity>>;
    fn add(entity: Entity);
    fn remove(entity: &Entity);
    fn handle_window_resize(old_window_size: Vector2<f32>, new_window_size: Vector2<f32>);
    fn update_spatial_hash(collider: &Collider);
    fn collide_check_rect(rect: Rectangle, layer_mask: i32);
    fn collide_check_split(x: f32, y: f32, w: f32, h: f32, layer_mask: i32);
    fn collide_check_entity(entity: &Entity, layer_mask: i32);
    //void CollideAll(Entity *e, std::vector<Colliders::Collider *> *collidingColliders, int layerMask = -1);
    //void CollideWith(Colliders::Collider *c, std::vector<Colliders::Collider *> *collidingColliders, int layerMask);
    //void RayCast(Ray2 *ray, std::vector<Colliders::RayCollisionData *> *rayCollisionData, int layerMask);
}

pub struct Scene {
    time_active: f32,
    focused: bool,
    world: World,
    //tag_lists: TagLists,
    //helper_entity: Entity,
    //spatial_hash: SpatialHash,
    actual_depth_lookup: HashMap<i32, f32>,
    tmp_rect: Rectangle,
    colliding_bodies: Vec<Rc<Collider>>,
    renderers: Vec<Rc<Renderer>>,
}

impl Scene {
    pub fn new(cell_size: u32) -> Self {
        let s = Scene {
            time_active: 0.0,
            focused: false,
            world: World::new(),
            actual_depth_lookup: HashMap::new(),
            tmp_rect: Rectangle::new(0.0, 0.0, 0, 0),
            colliding_bodies: Vec::new(),
            renderers: Vec::new(),
        };
        s
    }

    pub fn create_entity(&mut self) -> Entity {
        self.world.create_entity()
    }

    pub fn render_entities(&self) {
        //self.world.render_entities();
    }

    pub fn add_renderer(&mut self, renderer: Rc<Renderer>) {
        self.renderers.push(renderer);
    }

    pub fn begin(&mut self) {
        self.focused = true;
        //for entity in self.world.get_entities() {
            //entity.scene_begin();
        //}
    }

    pub fn end(&mut self) {
        self.focused = false;
        //for entity in self.world.get_entities() {
            //entity.scene_end();
        //}
    }

    pub fn before_update(&mut self) {
        //timeActive += Game::deltaTime;

        //self.world.update_lists();
        //tagLists.UpdateLists();
    }

    pub fn update(&self) {
        //self.world.update();
    }

    pub fn after_update(&self) {

    }

    pub fn before_render(&self) {
        for renderer in &self.renderers {
            renderer.before_render(self);
        }
    }

    pub fn render(&self) {
        for renderer in &self.renderers {
            //renderer.render(self);
        }
    }

    pub fn after_render(&self) {
        for renderer in &self.renderers {
            renderer.after_render(self);
        }
    }

    pub fn register_component<C: Component>(&mut self) {
        self.world.register_component_with_storage::<C>();
    }

    pub fn add_component_to_entity<C: Component>(&mut self, entity: Entity, component: C) {
        self.world.add_component_to_entity(entity, component);
    }

    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        return self.world.get_component_for_entity::<C>(entity);
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.world.destroy_entity(entity);
    }

    pub fn add_system<S: System>(&mut self, system: S) {
        self.world.add_system(system);
    }

    pub fn process(&self, dt: f32, user_data: &SystemData) {
        self.world.process(dt, user_data);
    } 
}