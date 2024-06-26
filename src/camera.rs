use nalgebra_glm::{half_pi, RealNumber};
use winit::event::{ElementState, MouseButton};

use crate::app::{Key, Position, Size};

pub struct PerspectiveParams{
    pub aspect: f32,
    pub fovy: f32,
    pub near: f32,
    pub far: f32
}

pub struct OrthographicParams{
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32
}

//Types of projections
pub enum ProjectionMatrix{
    Perspective(PerspectiveParams),
    Orthographic(OrthographicParams)
}

pub struct Camera{
    pub eye: nalgebra_glm::Vec3,
    pub up: nalgebra_glm::Vec3,
    pub right: nalgebra_glm::Vec3,
    pub forward: nalgebra_glm::Vec3,
    pub center: nalgebra_glm::Vec3,
    projection_matrix: nalgebra_glm::Mat4
}

impl Default for Camera{
    fn default() -> Self {
        Self { 
            eye: nalgebra_glm::Vec3::new(0.0,0.0,0.0), 
            up: nalgebra_glm::Vec3::new(0.0,1.0,0.0), 
            right: nalgebra_glm::Vec3::new(1.0,0.0,0.0), 
            forward: nalgebra_glm::Vec3::new(0.0,0.0,-1.0), 
            center: nalgebra_glm::Vec3::new(0.0,0.0,0.0) + nalgebra_glm::Vec3::new(0.0,0.0,-1.0),
            projection_matrix: nalgebra_glm::perspective(1.0, 45.0, 0.1, 100.0) 
        }
    }
}

impl Camera{
    pub fn new(
        projection: ProjectionMatrix
    ) -> Self{
        let projection_matrix = match projection{
            ProjectionMatrix::Perspective(PerspectiveParams{aspect,fovy,near,far}) => {
                nalgebra_glm::perspective(aspect, fovy, near, far)
            },
            ProjectionMatrix::Orthographic(OrthographicParams{left,right,top,bottom,near,far}) => {
                nalgebra_glm::ortho(left, right, bottom, top, near, far)
            },
        };
        Self{
            projection_matrix,
            ..Default::default()
        }
    }

    pub fn get_view_proj_matrix(&self) -> nalgebra_glm::Mat4{
        self.projection_matrix * nalgebra_glm::look_at(&self.eye,&self.center,&self.up)
    }
}

//Types of camera controllers

//Is like an fps flying camera
pub struct FlyCameraController{
    theta: f32,
    phi: f32,
    rotate_speed: f32,
    translate_speed: f32,

    //input state
    left_pressed: bool,
    right_pressed: bool,
    front_pressed: bool,
    back_pressed: bool,
    mouse_moved: bool,
    mouse_pressed: bool,
}

impl Default for FlyCameraController{
    fn default() -> Self {
        Self { 
            theta: half_pi(), 
            phi: half_pi(), 
            rotate_speed: 0.01, 
            translate_speed: 0.05,
            left_pressed: false,
            right_pressed: false,
            front_pressed: false,
            back_pressed: false,
            mouse_moved: false,
            mouse_pressed: false
        }
    }
}

impl FlyCameraController{
    pub fn new(rotate_speed: f32,translate_speed: f32) -> Self{
        FlyCameraController{
            rotate_speed,
            translate_speed,
            ..Default::default()
        }
    }

    pub fn on_key_fn(&mut self,key: Key,key_state: ElementState){
        match key{
            winit::keyboard::KeyCode::KeyW => {
                match key_state{
                    ElementState::Pressed => self.front_pressed = true,
                    ElementState::Released => self.front_pressed = false,
                }
            }
            winit::keyboard::KeyCode::KeyA => {
                match key_state{
                    ElementState::Pressed => self.left_pressed = true,
                    ElementState::Released => self.left_pressed = false,
                }
            }
            winit::keyboard::KeyCode::KeyS => {
                match key_state{
                    ElementState::Pressed => self.back_pressed = true,
                    ElementState::Released => self.back_pressed = false,
                }
            }
            winit::keyboard::KeyCode::KeyD => {
                match key_state{
                    ElementState::Pressed => self.right_pressed = true,
                    ElementState::Released => self.right_pressed = false,
                }
            }
            _ => {}
        }
    }

    pub fn on_mouse_move_fn(&mut self,delta: (f32,f32)){  
        if self.mouse_pressed{
            self.theta += delta.0 * self.rotate_speed;
        }
    }

    pub fn on_mouse_input_fn(&mut self,state: ElementState, button: MouseButton){
        match button{
            MouseButton::Left => {
                match  state{
                    ElementState::Pressed => self.mouse_pressed = true,
                    ElementState::Released => self.mouse_pressed = false,
                }
            }
            _ => {}
        }
    }

    pub fn update_camera(&mut self,camera: &mut Camera){

        if self.front_pressed {
            camera.eye = camera.eye +  camera.forward * self.translate_speed;
            camera.center = camera.eye + camera.forward;
        }

        if self.back_pressed{
            camera.eye -= camera.forward * self.translate_speed;
            camera.center = camera.eye + camera.forward;
        }

        if self.right_pressed{
            camera.eye += camera.right * self.translate_speed;
            camera.center = camera.eye + camera.forward;

        }

        if self.left_pressed{
            camera.eye -= camera.right * self.translate_speed;
            camera.center = camera.eye + camera.forward;
        }

        let x = f32::sin(self.phi) * f32::cos(self.theta);
        let y = f32::cos(self.phi);
        let z = f32::sin(self.phi) * f32::sin(self.theta);

        camera.forward = nalgebra_glm::vec3(x,y,z);
        //recompute frame
        camera.center = camera.eye + camera.forward;
        camera.right = nalgebra_glm::normalize(&camera.forward.cross(&camera.up));

    }
}