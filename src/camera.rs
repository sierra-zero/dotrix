
use crate::{
    ecs::{ Mut, Const },
    input::*,
};

const SPEED_ANGLE: f32 = 0.001;
const SPEED_DIST: f32 = 1.0;


pub struct Camera {
    distance: f32,
    yaw_angle: f32,
    pitch_angle: f32,
    view: cgmath::Matrix4<f32>,
}

pub fn camera_control(mut camera: Mut<Camera>, input: Const<InputManager>) {
    
    let mut delta_yaw = 0.0;
    let mut delta_pitch = 0.0;

    if input.get_button(Action::MoveLeft){
        delta_yaw += SPEED_ANGLE; 
    }

    if input.get_button(Action::MoveRight){
        delta_yaw -= SPEED_ANGLE; 
    }

    if input.get_button(Action::MoveForward){
        delta_pitch += SPEED_ANGLE; 
    }

    if input.get_button(Action::MoveBackward){
        delta_pitch -= SPEED_ANGLE; 
    }
    
    let distance = 10.0 + SPEED_DIST * input.get_scroll();
    let yaw_angle = camera.yaw_angle() + delta_yaw;
    let pitch_angle = camera.pitch_angle() + delta_pitch;

    let target = cgmath::Point3::new(0.0, 0.0, 0.0);

    camera.set(target, distance, yaw_angle, pitch_angle);
}

impl Camera {
    pub fn new(distance: f32, yaw_angle: f32, pitch_angle: f32) -> Self {
        Self {
            distance,
            yaw_angle,
            pitch_angle,
            view: Self::matrix(cgmath::Point3::new(0.0, 0.0, 0.0), distance, yaw_angle, pitch_angle),
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn yaw_angle(&self) -> f32 {
        self.yaw_angle
    }

    pub fn pitch_angle(&self) -> f32 {
        self.pitch_angle
    }

    pub fn view(&self) -> &cgmath::Matrix4<f32> {
        &self.view
    }

    pub fn set(&mut self, target: cgmath::Point3<f32>, distance: f32, yaw_angle: f32, pitch_angle: f32) {
        self.distance = distance;
        self.yaw_angle = yaw_angle;
        self.pitch_angle = pitch_angle;
        self.view = Self::matrix(target, distance, yaw_angle, pitch_angle);
    }

    pub fn look_at(&mut self, target: cgmath::Point3<f32>) {
        self.view = Self::matrix(target, self.distance, self.yaw_angle, self.pitch_angle);
    }

    fn matrix(
        target: cgmath::Point3<f32>,
        distance: f32,
        yaw_angle: f32,
        pitch_angle: f32
    ) -> cgmath::Matrix4<f32> {
        let dy = distance + pitch_angle.cos();
        let dx = distance * pitch_angle.sin() * yaw_angle.sin();
        let dz = distance * pitch_angle.sin() * yaw_angle.cos();
        cgmath::Matrix4::look_at(
            cgmath::Point3::new(target.x + dx, target.y + dy, target.z + dz),
            target,
            cgmath::Vector3::unit_y(),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(10.0, 3.14 / 2.0, 3.14 / 4.0)
    }
}
