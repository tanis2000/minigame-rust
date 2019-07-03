extern crate cgmath;

use rectangle::Rectangle;
use self::cgmath::{InnerSpace, MetricSpace, Vector2};
use utils::{Clamp};

pub struct CircleCollider {
    radius: f32,
    origin: Vector2<f32>,
}

impl CircleCollider {
    pub fn new(radius: f32, origin: Vector2<f32>) -> Self {
        CircleCollider {
            radius: radius,
            origin: Vector2::new(origin.x, origin.y),
        }
    }

    pub fn get_absolute_position(&self) -> Vector2<f32> {
        self.origin
    }

    pub fn get_absolute_left(&self) -> f32 {
        self.origin.x - self.radius
    }

    pub fn get_absolute_right(&self) -> f32 {
        self.origin.x + self.radius
    }

    pub fn get_absolute_top(&self) -> f32 {
        self.origin.y - self.radius
    }

    pub fn get_absolute_bottom(&self) -> f32 {
        self.origin.y + self.radius
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

pub struct BoxCollider {
    origin: Vector2<f32>,
    width: f32,
    height: f32,
}

impl BoxCollider {
    pub fn new(origin: Vector2<f32>, width: f32, height: f32) -> Self {
        BoxCollider {
            origin: origin,
            width: width,
            height: height,
        }
    }

    pub fn get_absolute_position(&self) -> Vector2<f32> {
        self.origin
    }

    pub fn get_absolute_left(&self) -> f32 {
        self.origin.x
    }

    pub fn get_absolute_right(&self) -> f32 {
        self.origin.x + self.width
    }

    pub fn get_absolute_top(&self) -> f32 {
        self.origin.y
    }

    pub fn get_absolute_bottom(&self) -> f32 {
        self.origin.y + self.height
    }

    pub fn as_rect(&self) -> Rectangle {
        let rect = Rectangle::new(self.origin.x, self.origin.y, self.width as i32, self.height as i32);
        return rect;
    }

}

bitflags!{
    flags PointSectors: u8 {
        const CENTER = 0,
        const TOP = 1,
        const BOTTOM = 2,
        const TOP_LEFT = 9,
        const TOP_RIGHT = 5,
        const LEFT = 8,
        const RIGHT = 4,
        const BOTTOM_LEFT = 10,
        const BOTTOM_RIGHT = 6
    }
} 

pub struct Collider {

}

impl Collider {
    pub fn closest_point_on_line(line_a: Vector2<f32>, line_b: Vector2<f32>, colsest_to: Vector2<f32>) -> Vector2<f32> {
        let v = line_b - line_a;
        let w = colsest_to - line_a;
        let mut t: f32 = w.dot(v) / v.dot(v);
        t = t.clamp(0.0, 1.0);
        return line_a + v * t;
    }

    /*
    Bitflags and helpers for using the Cohen–Sutherland algorithm
    http://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm
 
    Sector bitflags:
    1001  1000  1010
    0001  0000  0010
    0101  0100  0110
    */
  
    pub fn get_sector_rect_point(rect: Rectangle, point: Vector2<f32>) -> PointSectors {
        let mut sector = CENTER;

        if point.x < rect.get_left() {
            sector = sector | LEFT;
        } else if point.x >= rect.get_right() {
            sector = sector | RIGHT;
        }

        if point.y < rect.get_top() {
            sector = sector | TOP;
        } else if point.y >= rect.get_bottom() {
            sector = sector | BOTTOM;
        }

        return sector;
    }

    pub fn get_sector_split_rect_point(r_x: f32, r_y: f32, r_w: f32, r_h: f32, point: Vector2<f32>) -> PointSectors {
        let mut sector = CENTER;

        if point.x < r_x {
            sector = sector | LEFT;
        } else if point.x >= r_x + r_w {
            sector = sector | RIGHT;
        }

        if point.y < r_y {
            sector = sector | TOP;
        } else if point.y >= r_y + r_h {
            sector = sector | BOTTOM;
        }

        return sector;
    }


    pub fn collide_circle_to_point(circle: &CircleCollider, point: Vector2<f32>) -> bool {
        let c_pos = circle.get_absolute_position();
        return c_pos.distance2(point) < circle.get_radius() * circle.get_radius();
    }

    pub fn collide_circle_to_line(circle: &CircleCollider, line_from: Vector2<f32>, line_to: Vector2<f32>) -> bool {
        let closest = Collider::closest_point_on_line(line_from, line_to, circle.get_absolute_position());
        return closest.distance2(circle.get_absolute_position()) < circle.get_radius() * circle.get_radius();
    }

    pub fn collide_circle_to_rect(circle: &CircleCollider, rect: Rectangle) -> bool {
        // Check if the circle contains the rectangle's center-point
        if Collider::collide_circle_to_point(circle, Vector2::new(rect.x + rect.w as f32 / 2.0, rect.y + rect.h as f32 / 2.0)) {
            return true;
        }

        // Check the circle against the relevant edges
        let mut edge_from: Vector2<f32>;
        let mut edge_to: Vector2<f32>;
        let sector = Collider::get_sector_rect_point(rect, circle.get_absolute_position());

        if sector.contains(TOP) {
            edge_from = Vector2::new(rect.x, rect.y);
            edge_to = Vector2::new(rect.x + rect.w as f32, rect.y);
            if Collider::collide_circle_to_line(circle, edge_from, edge_to) {
                return true;
            }
        }

        if sector.contains(BOTTOM) {
            edge_from = Vector2::new(rect.x, rect.y + rect.h as f32);
            edge_to = Vector2::new(rect.x + rect.w as f32, rect.y + rect.h as f32);
            if Collider::collide_circle_to_line(circle, edge_from, edge_to) {
                return true;
            }
        }

        if sector.contains(LEFT) {
            edge_from = Vector2::new(rect.x, rect.y);
            edge_to = Vector2::new(rect.x, rect.y + rect.h as f32);
            if Collider::collide_circle_to_line(circle, edge_from, edge_to) {
                return true;
            }
        }

        if sector.contains(RIGHT) {
            edge_from = Vector2::new(rect.x + rect.w as f32, rect.y);
            edge_to = Vector2::new(rect.x + rect.w as f32, rect.y + rect.h as f32);
            if Collider::collide_circle_to_line(circle, edge_from, edge_to) {
                return true;
            }
        }

        return false;
    }

    pub fn collide_circle_to_circle(circle1: &CircleCollider, circle2: &CircleCollider) -> bool {
        let c1_pos = circle1.get_absolute_position();
        let c2_pos = circle2.get_absolute_position();
        return c1_pos.distance2(c2_pos) < (circle1.get_radius() + circle2.get_radius()) * (circle1.get_radius() + circle2.get_radius())
    }

    pub fn collide_circle_to_box(circle: &CircleCollider, box_collider: &BoxCollider) -> bool {
        return Collider::collide_circle_to_rect(circle, box_collider.as_rect());
    }

    pub fn collide_box_to_box(box_collider1: &BoxCollider, box_collider2: &BoxCollider) -> bool {
        return box_collider1.get_absolute_left() < box_collider2.get_absolute_right() &&
        box_collider1.get_absolute_right() > box_collider2.get_absolute_left() &&
        box_collider1.get_absolute_bottom() > box_collider2.get_absolute_top() &&
        box_collider1.get_absolute_top() < box_collider2.get_absolute_bottom();
    }
}