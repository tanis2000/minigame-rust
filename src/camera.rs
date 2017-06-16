extern crate cgmath;

use self::cgmath::Matrix4;
use self::cgmath::Vector2;
use self::cgmath::Vector3;
use self::cgmath::One;
use self::cgmath::Zero;
use self::cgmath::Rad;
use self::cgmath::SquareMatrix;
use std::ops::Mul;
use utils::Clamp;
use rectangle::Rectangle;
use graphicsdevice::GraphicsDevice;
use viewportadapter::ViewportAdapter;
use viewportadapter::ViewportAdapterTrait;
use utils::MinMax;
use utils::mtx_mul_v;

pub enum CameraResizeType {
  CameraResizeTypeProportional,
  CameraResizeTypeFixed,
  CameraResizeTypePixelPerfect
}

pub struct Camera<T: ViewportAdapterTrait> {
    transform_matrix: Matrix4<f32>,
    inverse_transform_matrix: Matrix4<f32>,
    camera_resize_type: CameraResizeType,
    position: Vector2<f32>,
    origin: Vector2<f32>,
    rotation: f32,
    zoom: f32,
    min_zoom: f32,
    max_zoom: f32,
    are_matrixes_dirty: bool,
    are_bounds_dirty: bool,
    bounds: Rectangle,
    viewport_adapter: Option<T>,
    near: f32,
    far: f32,
}

impl<T: ViewportAdapterTrait> Camera<T> {
    pub fn new() -> Self {
        Camera {
            transform_matrix: Matrix4::one(),
            inverse_transform_matrix: Matrix4::one(),
            camera_resize_type: CameraResizeType::CameraResizeTypePixelPerfect,
            position: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            origin: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            rotation: 0.0,
            zoom: 1.0,
            min_zoom: 1.0,
            max_zoom: 1.0,
            are_matrixes_dirty: false,
            are_bounds_dirty: false,
            bounds: Rectangle::new(0.0, 0.0, 0, 0),
            viewport_adapter: None,
            near: -10.0,
            far: 10.0,
        }
    }

    pub fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
        self.force_matrix_update();
    }

    pub fn get_origin(&self) -> Vector2<f32> {
        return self.origin
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        self.origin.x = x;
        self.origin.y = y;
        self.force_matrix_update();
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.force_matrix_update();
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(self.min_zoom, self.max_zoom);
        self.are_matrixes_dirty = true;
    }

    pub fn get_min_zoom(&self) -> f32 {
        self.min_zoom
    }
    
    pub fn set_min_zoom(&mut self, min_zoom: f32) {
        if self.zoom < min_zoom {
            self.set_zoom(min_zoom);
        }
        self.min_zoom = min_zoom;
    }

    pub fn get_max_zoom(&self) -> f32 {
        self.max_zoom
    }
    
    pub fn set_max_zoom(&mut self, max_zoom: f32) {
        if self.zoom > max_zoom {
            self.set_zoom(max_zoom);
        }
        self.max_zoom = max_zoom;
    }
    
    pub fn get_bounds(&mut self) -> Rectangle {
        if self.are_matrixes_dirty {
            self.update_matrixes();
        }
        
        if self.are_bounds_dirty {
            // top-left and bottom-right are needed by either rotated or non-rotated bounds
            let topLeft = self.screen_to_world_point(
                Vector2::new(self.viewport_adapter.as_ref().unwrap().get_viewport().x, self.viewport_adapter.as_ref().unwrap().get_viewport().y)
                );
            let bottomRight = self.screen_to_world_point(
                Vector2::new(self.viewport_adapter.as_ref().unwrap().get_viewport().x + self.viewport_adapter.as_ref().unwrap().get_viewport().w as f32, 
                self.viewport_adapter.as_ref().unwrap().get_viewport().y + self.viewport_adapter.as_ref().unwrap().get_viewport().h as f32) 
                );
            
            if self.rotation != 0.0
            {
                // special care for rotated bounds. we need to find our absolute min/max values and create the bounds from that
                let topRight = self.screen_to_world_point( Vector2::new( self.viewport_adapter.as_ref().unwrap().get_viewport().x + self.viewport_adapter.as_ref().unwrap().get_viewport().w as f32, self.viewport_adapter.as_ref().unwrap().get_viewport().y ) );
                let bottomLeft = self.screen_to_world_point( Vector2::new( self.viewport_adapter.as_ref().unwrap().get_viewport().x, self.viewport_adapter.as_ref().unwrap().get_viewport().y + self.viewport_adapter.as_ref().unwrap().get_viewport().h as f32 ) );
                
                let minX = f32::min_of( topLeft.x, bottomRight.x, topRight.x, bottomLeft.x );
                let maxX = f32::max_of( topLeft.x, bottomRight.x, topRight.x, bottomLeft.x );
                let minY = f32::min_of( topLeft.y, bottomRight.y, topRight.y, bottomLeft.y );
                let maxY = f32::max_of( topLeft.y, bottomRight.y, topRight.y, bottomLeft.y );
                
                self.bounds.x = minX;
                self.bounds.y = minY;
                self.bounds.w = ( maxX - minX ) as i32;
                self.bounds.h = ( maxY - minY ) as i32;
            }
            else
            {
                self.bounds.x = topLeft.x;
                self.bounds.y = topLeft.y;
                self.bounds.w = ( bottomRight.x - topLeft.x ) as i32;
                self.bounds.h = ( bottomRight.y - topLeft.y ) as i32;
            }
            
            self.are_bounds_dirty = false;
        }
        return self.bounds;
    }

    pub fn get_transform_matrix(&mut self) -> Matrix4<f32> {
        if self.are_matrixes_dirty {
            self.update_matrixes();
        }
        self.transform_matrix
    }

    pub fn get_inverse_transform_matrix(&mut self) -> Matrix4<f32> {
        if self.are_matrixes_dirty {
            self.update_matrixes();
        }
        self.inverse_transform_matrix
    }
    
    pub fn get_viewport_adapter(&self) -> &Option<T> {
        &self.viewport_adapter
    }
    
    pub fn set_viewport_adapter(&mut self, viewport_adapter: Option<T>) {
        if self.viewport_adapter.is_none() || self.viewport_adapter.as_ref().unwrap() as *const _ != viewport_adapter.as_ref().unwrap() as *const _ {
            self.viewport_adapter = viewport_adapter;
            self.are_matrixes_dirty = true;
            self.are_bounds_dirty = true;
            // TODO: should we update both matrixes and bounds automatically?
        }
    }

    pub fn update_matrixes(&mut self) {
        let mut tempMat: Matrix4<f32> = Matrix4::zero();
        
        self.transform_matrix = Matrix4::from_translation(Vector3::new(-self.position.x, -self.position.y, 0.0)); // position
        tempMat = Matrix4::from_nonuniform_scale(self.zoom, self.zoom, 1.0); // scale
        self.transform_matrix = self.transform_matrix.mul(tempMat);
        tempMat = Matrix4::from_angle_z(Rad(self.rotation)); // rotation
        self.transform_matrix = self.transform_matrix.mul(tempMat);
        // TODO: clamp origin to integer values, see -> tempMat.CreateTranslation( (int)origin.x, (int)origin.y, 0.0f );
        tempMat = Matrix4::from_translation(Vector3::new(self.origin.x, self.origin.y, 0.0)); // translate -origin
        self.transform_matrix = self.transform_matrix.mul(tempMat);
        
        // if we have a ViewportAdapter take it into account
        if self.viewport_adapter.is_some() {
            self.transform_matrix = self.transform_matrix.mul(self.viewport_adapter.as_ref().unwrap().get_scale_matrix());
        }
        
        // calculate our inverse as well
        self.inverse_transform_matrix = self.transform_matrix.invert().unwrap();
        
        // whenever the matrix changes the bounds are then invalid
        self.are_bounds_dirty = true;
        self.are_matrixes_dirty = false;
    }
    
    
    /// <summary>
    /// this forces the matrix and bounds dirty
    /// </summary>
    pub fn force_matrix_update(&mut self) {
        self.are_matrixes_dirty = true;
        self.are_bounds_dirty = true;
    }
    
    pub fn round_position(&mut self) {
        let x = self.position.x.round();
        let y = self.position.y.round();
        self.set_position(x, y);
        self.are_matrixes_dirty = true;
    }
    
    
    pub fn center_origin(&mut self) {
        if self.viewport_adapter.is_some() {
            let o = Vector2::new( self.viewport_adapter.as_ref().unwrap().get_virtual_width() as f32 / 2.0, self.viewport_adapter.as_ref().unwrap().get_virtual_height() as f32 / 2.0 );
            self.set_origin(o.x, o.y);
        }
        else {
            panic!("Missing ViewportAdapter!");
            // TODO: something like the following instead of panicking
            //origin = Vector2( _graphicsDevice.Viewport.Width / 2.0f, _graphicsDevice.Viewport.Height / 2f );
        }
        
        // offset our position to match the new center
        let x = self.position.x + self.origin.x;
        let y = self.position.y + self.origin.y;
        self.set_position(x, y);
    }
    
    
    pub fn translate(&mut self, direction: Vector2<f32> ) {
        //position += Vector2.Transform( direction, Matrix.CreateRotationZ( -rotation ) );
    }
    
    
    pub fn rotate(&mut self, delta_radians: f32 ) {
        let r = self.rotation + delta_radians;
        self.set_rotation(r);
    }
    
    pub fn zoom_in(&mut self, delta_zoom: f32)
    {
        let z = self.zoom + delta_zoom;
        self.set_zoom(z);
    }
    
    
    pub fn zoom_out(&mut self, delta_zoom: f32)
    {
        let z = self.zoom - delta_zoom;
        self.set_zoom(z);
    }
    
    
    /// <summary>
    /// converts a point from world coordinates to screen
    /// </summary>
    /// <returns>The to screen point.</returns>
    /// <param name="worldPosition">World position.</param>
    pub fn world_to_screen_point(&self, world_position: Vector2<f32>) -> Vector2<f32>
    {
        let mut pos = mtx_mul_v(self.transform_matrix, world_position);
        
        if self.viewport_adapter.is_some() {
            pos = self.viewport_adapter.as_ref().unwrap().screen_to_virtual_viewport(pos);
        }

        pos
    }
    
    
    /// <summary>
    /// converts a point from screen coordinates to world
    /// </summary>
    /// <returns>The to world point.</returns>
    /// <param name="screenPosition">Screen position.</param>
    pub fn screen_to_world_point(&self, screen_position: Vector2<f32>) -> Vector2<f32>
    {
        let mut pos = screen_position;
        if self.viewport_adapter.is_some() {
            pos = self.viewport_adapter.as_ref().unwrap().point_to_virtual_viewport(screen_position);
        }
        mtx_mul_v(self.inverse_transform_matrix, pos)
    }
    
    
    /// <summary>
    /// gets this cameras project matrix
    /// </summary>
    /// <returns>The projection matrix.</returns>
    pub fn get_projection_matrix(&self) -> Matrix4<f32>
    {
        // not currently blocked with a dirty flag due to the core engine not using this
        GraphicsDevice::createOrthographicMatrixOffCenter( 0.0, self.viewport_adapter.as_ref().unwrap().get_viewport().w as f32, self.viewport_adapter.as_ref().unwrap().get_viewport().h as f32, 0.0, self.near, self.far )
    }
    
    
    /// <summary>
    /// gets the view-projection matrix which is the transformMatrix * the projection matrix
    /// </summary>
    /// <returns>The view projection matrix.</returns>
    pub fn get_view_projection_matrix(&self) -> Matrix4<f32>
    {
        // not currently blocked with a dirty flag due to the core engine not using this
        self.transform_matrix.mul(self.get_projection_matrix())
    }
    

    pub fn handle_window_resize(self, old_window_size: Vector2<f32>, new_window_size: Vector2<f32>) {
        if self.viewport_adapter.is_some() {
            self.viewport_adapter.unwrap().reset();
        }
    }

}



