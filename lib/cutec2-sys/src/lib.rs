/* automatically generated by rust-bindgen */
extern crate nalgebra as na;
mod bindings;
pub mod cutec2 {
  //This is all very game specific (could be made more general later)
    use super::bindings;
   /* static XVector Rotate(XVector point, float angle, XVector center_of_rotation)
{
	float sinX = sin(angle);
	float cosX = cos(angle);
	XVector temppoint = XVector(point.x - center_of_rotation.x, point.y - center_of_rotation.y);
	XVector RotatedPoint = XVector(temppoint.x * cosX - (temppoint.y * sinX), temppoint.x * sinX + temppoint.y * cosX);

	return RotatedPoint+center_of_rotation;
}*/

    fn rotate_around_point(point : &na::Vector2<f32>,angle : f32, center_of_rotation : &na::Vector2<f32>) -> na::Vector2<f32>{
        let sin_x = angle.sin();
        let cos_x = angle.cos();
        let temppoint = point - center_of_rotation;
        let rotated_point = na::Vector2::<f32>::new(temppoint.x * cos_x - (temppoint.y * sin_x),temppoint.x * sin_x + temppoint.y * cos_x);
        rotated_point + center_of_rotation
    }
    fn from_vector2_to_c2v(input: &na::Vector2<f32>) -> bindings::c2v {
        bindings::c2v {
            x: input.x,
            y: input.y,
        }
    }
    fn from_c2v_to_vector2(input :&bindings::c2v) -> na::Vector2<f32> {
      na::Vector2::<f32>::new(
          input.x,input.y
      )
    }
    //Makes us an AABB Box (mostly will be used for the player)
    pub fn make_aabb(position: &na::Vector2<f32>, scale: &na::Vector2<f32>) -> bindings::c2AABB {
        bindings::c2AABB {
            min: from_vector2_to_c2v(&(position - (scale / 2.0))),
            max: from_vector2_to_c2v(&(position + (scale / 2.0))),
        }
    }
    pub fn make_poly(
        position: &na::Vector2<f32>,
        scale: &na::Vector2<f32>,
        rotation: f32,
    ) -> bindings::c2Poly {
        let mut poly: bindings::c2Poly = bindings::c2Poly {
            verts: [bindings::c2v { x: 0.0, y: 0.0 }; 8],
            norms: [bindings::c2v { x: 0.0, y: 0.0 }; 8],
            count: 4,
        };
        // Top Left
        poly.verts[0] = bindings::c2v {
            x: position.x - scale.x / 2.0,
            y: position.y + scale.y / 2.0,
        };

        // Bottom Left
        poly.verts[1] = bindings::c2v {
            x: position.x - scale.x / 2.0,
            y: position.y - scale.y / 2.0,
        };

        // Bottom Right
        poly.verts[2] = bindings::c2v {
            x: position.x + scale.x / 2.0,
            y: position.y - scale.y / 2.0,
        };

        // Top Right
        poly.verts[3] = bindings::c2v {
            x: position.x + scale.x / 2.0,
            y: position.y + scale.y / 2.0,
        };
        for x in 0 .. 4{

            let dealer = from_vector2_to_c2v( &rotate_around_point(&from_c2v_to_vector2(&poly.verts[x]), rotation, &position));
            poly.verts[x] = dealer;
        }
        poly.count = 4;
        unsafe {
            bindings::c2Norms(
                poly.verts.as_ptr() as *mut bindings::c2v,
                poly.norms.as_ptr() as *mut bindings::c2v,
                poly.count,
            );
        }
        poly
    }
    pub fn check_collision(position : &na::Vector2<f32>,scale : &na::Vector2<f32>, testagainst : &bindings::c2Poly) -> (f32,na::Vector2<f32>) {
      let player = make_aabb(&position, &scale);
      let mut manifold : bindings::c2Manifold = bindings::c2Manifold{count : 0,depths : [0.0;2],contact_points : [bindings::c2v { x: 0.0, y: 0.0 };2],n:bindings::c2v { x: 0.0, y: 0.0 }};
       
      unsafe{
        bindings::c2Collide(testagainst as *const _ as *const std::os::raw::c_void, std::ptr::null(), bindings::C2_TYPE_C2_TYPE_POLY, &player as *const _ as *const std::os::raw::c_void, std::ptr::null(), bindings::C2_TYPE_C2_TYPE_AABB,&mut manifold);
      }
      (manifold.depths[0],from_c2v_to_vector2(&manifold.n))
    }
    pub fn sweep_test(position : &na::Vector2<f32>,scale: &na::Vector2<f32>, velocity : &na::Vector2<f32>, testagainst : &bindings::c2Poly) -> (f32,na::Vector2<f32>,na::Vector2<f32>) {
      let player = make_aabb(&position, &scale);
      let mut normal : bindings::c2v = bindings::c2v{x:0.0,y:0.0}; 
      let mut contact : bindings::c2v = bindings::c2v{x:0.0,y:0.0}; 
      let mut iters = 0;
      let  toi =  unsafe{
         bindings::c2TOI( testagainst as *const _ as *const std::os::raw::c_void, bindings::C2_TYPE_C2_TYPE_POLY, std::ptr::null(), from_vector2_to_c2v(&na::Vector2::<f32>::zeros()), &player as *const _ as *const std::os::raw::c_void, bindings::C2_TYPE_C2_TYPE_AABB, std::ptr::null(), from_vector2_to_c2v(velocity),1, &mut normal as *mut _ as *mut bindings::c2v, &mut contact as *mut _ as *mut bindings::c2v, &mut iters as *mut std::os::raw::c_int)
      };
      (toi,from_c2v_to_vector2(&normal),from_c2v_to_vector2(&contact))
    }
    pub fn raycast(position : &na::Vector2<f32>,direction : &na::Vector2<f32>,distance : f32,testagainst : &bindings::c2Poly) -> std::option::Option<(f32,na::Vector2<f32>)> {
        let ray : bindings::c2Ray = bindings::c2Ray{p : from_vector2_to_c2v(position),d : from_vector2_to_c2v(direction),t : distance};
        let mut raycastout : bindings::c2Raycast = bindings::c2Raycast{t:0.0,n: bindings::c2v{x:0.0,y:0.0}};
        unsafe{
        let hit = bindings::c2RaytoPoly(ray, testagainst as *const _ , std::ptr::null(), &mut raycastout as *mut _ );
        if hit != 0{
            return Some((raycastout.t,from_c2v_to_vector2(&raycastout.n)))
        }else {
            return None
        } 
        }
        
    }
}


