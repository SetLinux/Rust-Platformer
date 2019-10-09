use specs::{Component, VecStorage};
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: na::Vector3<f32>,
    pub rotation: f32,
    pub scale: na::Vector3<f32>,
}
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub tex_id: gl::types::GLuint,
    pub coords : na::Vector4<f32>
}
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Movement {
    pub velocity: na::Vector3<f32>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collider {
    pub body_handle: i16,
    pub position: na::Vector3<f32>,
    pub rotation: f32,
    pub scale: na::Vector3<f32>,
    pub vel: na::Vector3<f32>,
    pub slope: bool,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {
    pub health: f32,
    pub vel: na::Vector3<f32>,
    pub jump: bool,
}
