extern crate cgmath;

use entity::Entity;
use entitylist::EntityList;
use collider::Collider;
use rectangle::Rectangle;
use renderer::Renderer;
use std::vec::Vec;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::boxed::Box;
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
    entities: EntityList,
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
            entities: EntityList::new(),
            actual_depth_lookup: HashMap::new(),
            tmp_rect: Rectangle::new(0.0, 0.0, 0, 0),
            colliding_bodies: Vec::new(),
            renderers: Vec::new(),
        };
        s
    }

    pub fn create_entity(&mut self) -> u32 {
        self.entities.create_entity()
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.add(entity)
    }

    pub fn render_entities(&self) {
        self.entities.render_entities();
    }

    pub fn add_renderer(&mut self, renderer: Rc<Renderer>) {
        self.renderers.push(renderer);
    }

    pub fn get_entity_mut(&mut self, entity_id: u32) -> Option<&mut Entity> {
        self.entities.get_entity_mut(entity_id)
    }

    pub fn begin(&mut self) {
        self.focused = true;
        for entity in self.entities.get_entities() {
            entity.scene_begin();
        }
    }

    pub fn end(&mut self) {
        self.focused = false;
        for entity in self.entities.get_entities() {
            entity.scene_end();
        }
    }

    pub fn before_update(&mut self) {
        //timeActive += Game::deltaTime;

        self.entities.update_lists();
        //tagLists.UpdateLists();
    }

    pub fn update(&self) {
        self.entities.update();
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
            renderer.render(self);
        }
    }

    pub fn after_render(&self) {
        for renderer in &self.renderers {
            renderer.after_render(self);
        }
    }
}