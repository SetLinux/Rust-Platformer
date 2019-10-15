use super::components;
use super::graphics;
use specs::Builder;
//Just draws a line between two points
fn add_line_between_two_points(world : &mut specs::World,pointa : na::Vector2<f32>,pointb : na::Vector2<f32>,thickness : f32,texid : gl::types::GLuint){
    let midpoint = (pointa + pointb) / 2.0;
    let scale = na::Matrix::magnitude(&(pointb - pointa));
    let x_coord = pointa.x - pointb.x;
    let y_coords = pointa.y - pointb.y;
    let tan_value = y_coords / x_coord;
    let rotaiton = tan_value.atan();

    world
        .create_entity()
        .with(components::Transform {
            position: na::Vector3::<f32>::new(midpoint.x, midpoint.y, 0.0),
            rotation: rotaiton,
            scale: na::Vector3::<f32>::new(scale / 2.0, thickness, 1.0),
        })
        .with(components::Sprite { tex_id: texid ,coords : na::Vector4::<f32>::new(0.0,0.0,0.0,0.0)}).with(components::Collider {
        body_handle: -1,
        position:  (na::Vector3::<f32>::new(midpoint.x, midpoint.y, 0.0)),
        rotation: rotaiton,
        scale: (na::Vector3::<f32>::new(scale / 2.0, thickness, 1.0)),
        vel: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
        slope: false,
    }).build();

}
pub struct LinesSystem{
    pub drawing : bool,
    pub start_point : na::Vector2<f32>,
    pub end_point : na::Vector2<f32>
}
impl LinesSystem {
    pub fn updatelinesystem(&mut self,world : &mut specs::World,mouse_xpos : f32,mouse_ypos : f32,mouse_clicked : bool,texid : gl::types::GLuint) {
        if self.drawing && mouse_clicked {
            self.end_point = na::Vector2::<f32>::new(mouse_xpos,mouse_ypos);
            if na::Matrix::magnitude (&(self.end_point - self.start_point) ) > 60.0 {
                add_line_between_two_points(world, self.start_point, self.end_point, 12.0, texid);
                self.start_point = na::Vector2::<f32>::zeros();
                self.drawing = false;

            }else{

            }
        }else  if !self.drawing  && mouse_clicked {
            println!("just strated drawing");
            self.drawing = true;
            self.start_point = na::Vector2::<f32>::new(mouse_xpos,mouse_ypos);
        }
    }
}