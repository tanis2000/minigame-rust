extern crate cgmath;

use self::cgmath::Vector2;
use self::cgmath::Matrix4;
use self::cgmath::One;
use rectangle::Rectangle;

pub trait ViewportAdapterTrait {
    fn new() -> Self;
    fn with_size(original_width: i32, original_height: i32) -> Self;
    fn with_size_and_virtual(original_width: i32, original_height: i32, virtual_width: i32, virtual_height: i32) -> Self;
    fn get_virtual_width(&self) -> i32;
    fn get_virtual_height(&self) -> i32;
    fn get_viewport_width(&self) -> i32;
    fn get_viewport_height(&self) -> i32;
    fn point_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>;
    fn screen_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>;
    fn reset(&mut self);
    fn set_viewport(&mut self, viewport: Rectangle);
    fn get_viewport(&self) -> Rectangle;
    fn get_original_viewport(&self) -> Rectangle;
    fn get_scale_matrix(&self) -> Matrix4<f32>;
}

pub struct ViewportAdapter {
    viewport: Rectangle,
    original_viewport: Rectangle,
    scale_matrix: Matrix4<f32>,
}

impl ViewportAdapterTrait for ViewportAdapter {
    fn new() -> Self {
        ViewportAdapter {
            viewport: Rectangle::new(0.0, 0.0, 0, 0),
            original_viewport: Rectangle::new(0.0, 0.0, 0, 0),
            scale_matrix: Matrix4::one(),
        }
    }

    fn with_size(original_width: i32, original_height: i32) -> Self {
        ViewportAdapter {
            viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            original_viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            scale_matrix: Matrix4::one(),
        }
    }

    fn with_size_and_virtual(original_width: i32, original_height: i32, virtual_width: i32, virtual_height: i32) -> Self {
        ViewportAdapter {
            viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            original_viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            scale_matrix: Matrix4::one(),
        }
    }

    fn get_virtual_width(&self) -> i32 {
        self.original_viewport.w
    }

    fn get_virtual_height(&self) -> i32 {
        self.original_viewport.h
    }

    fn get_viewport_width(&self) -> i32 {
        self.viewport.w
    }

    fn get_viewport_height(&self) -> i32 {
        self.viewport.h
    }

    fn point_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>
    {
        return point;
    }
    
    fn screen_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>
    {
        return point;
    }
        
    fn reset(&mut self) {
    }
  
    fn set_viewport(&mut self, viewport: Rectangle) {
        self.viewport = viewport;
        self.original_viewport = viewport;
    }
    
    fn get_viewport(&self) -> Rectangle {
        self.viewport
    }

    fn get_original_viewport(&self) -> Rectangle {
        self.original_viewport
    }

    fn get_scale_matrix(&self) -> Matrix4<f32> {
        self.scale_matrix
    }

}

pub struct ScalingViewportAdapter {
    base: ViewportAdapter,
    virtual_width: i32,
    virtual_height: i32,
}

impl ViewportAdapterTrait for ScalingViewportAdapter {
    fn new() -> Self {
        ScalingViewportAdapter {
            base: ViewportAdapter::new(),
            virtual_width: 0,
            virtual_height: 0,
        }
    }

    fn with_size(original_width: i32, original_height: i32) -> Self {
        ScalingViewportAdapter {
            base: ViewportAdapter::with_size(original_width, original_height),
            virtual_width: 0,
            virtual_height: 0,
        }
    }

    fn with_size_and_virtual(original_width: i32, original_height: i32, virtual_width: i32, virtual_height: i32) -> Self {
        let mut s = ScalingViewportAdapter {
            base: ViewportAdapter::with_size(original_width, original_height),
            virtual_width: virtual_width,
            virtual_height: virtual_height,
        };
        s.reset();
        s
    }

    fn get_virtual_width(&self) -> i32 {
        self.base.get_virtual_width()
    }

    fn get_virtual_height(&self) -> i32 {
        self.base.get_viewport_height()
    }

    fn get_viewport_width(&self) -> i32 {
        self.base.get_viewport_width()
    }

    fn get_viewport_height(&self) -> i32 {
        self.base.get_viewport_height()
    }

    fn point_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32> {
        self.base.point_to_virtual_viewport(point)
    }
    
    fn screen_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>
    {
        self.base.screen_to_virtual_viewport(point)
    }
        
    fn reset(&mut self) {
        let oldWindowSize = Vector2::new(self.virtual_width, self.virtual_height);
        let newWindowSize = Vector2::new(self.base.original_viewport.w, self.base.original_viewport.h);
        let ratioX = newWindowSize.x / oldWindowSize.x;
        let ratioY = newWindowSize.y / oldWindowSize.y;
        let originalViewport = self.get_original_viewport();
        self.base.viewport.x = originalViewport.x * ratioX as f32;
        self.base.viewport.y = originalViewport.y * ratioY as f32;
        self.base.viewport.w = originalViewport.w * ratioX;
        self.base.viewport.h = originalViewport.h * ratioY;
        let scaleX = (self.get_viewport_width() / self.virtual_width) as f32;
        let scaleY = (self.get_viewport_height() / self.virtual_height) as f32;
        self.base.scale_matrix = Matrix4::from_nonuniform_scale(scaleX, scaleY, 1.0);
    }
  
    fn set_viewport(&mut self, viewport: Rectangle) {
        self.base.set_viewport(viewport);
    }
    
    fn get_viewport(&self) -> Rectangle {
        self.base.get_viewport()
    }

    fn get_original_viewport(&self) -> Rectangle {
        self.base.get_original_viewport()
    }

    fn get_scale_matrix(&self) -> Matrix4<f32> {
        self.base.get_scale_matrix()
    }
}

