use super::components;
use specs::{Join, System, WriteStorage};

#[derive(Debug)]
pub struct PhysicsBody {
    pub position: na::Vector3<f32>,
    pub scale: na::Vector3<f32>,
    pub vel: na::Vector3<f32>,
    pub rotation: f32,
}
pub struct PhysicsSystem {
    pub m_bodies: Vec<PhysicsBody>,
}
impl PhysicsSystem {
    //this is a very sad hell
    pub fn raycast(
        &self,
        origin: na::Vector2<f32>,
        dir: na::Vector2<f32>,
        max_distance: f32,
    ) -> std::option::Option<(f32, na::Vector2<f32>)> {
        let mut min_toi: f32 = 10000.0;
        let mut min_normal: na::Vector2<f32> = na::Vector2::<f32>::new(0.0, 0.0);
        for testagainst in &self.m_bodies {
            let rc = cutec2_sys::cutec2::raycast(
                &origin,
                &dir,
                max_distance,
                &cutec2_sys::cutec2::make_poly(
                    &testagainst.position.xy(),
                    &(testagainst.scale.xy() * 2.0),
                    testagainst.rotation,
                ),
            );
            match rc {
                Some((x, y)) => {
                    if x < min_toi {
                        min_toi = x;
                        min_normal = y;
                    }
                }
                None => (),
            }
        }

        match min_toi {
            x if x != 10000.0 => Some((min_toi, min_normal)),
            _ => (None),
        }
    }

    pub fn on_slope(
        &self,
        pos: &na::Vector2<f32>,
        scale: &na::Vector2<f32>,
    ) -> (i8, na::Vector2<f32>) {
        let mut rd = false;
        let mut r = false;
        let mut l = false;
        let mut ld = false;

        let mut rd_toi = na::Vector2::<f32>::new(0.0, 0.0);
        let mut r_toi = na::Vector2::<f32>::new(0.0, 0.0);
        let mut l_toi = na::Vector2::<f32>::new(0.0, 0.0);
        let mut ld_toi = na::Vector2::<f32>::new(0.0, 0.0);

        let (leftgoingray, _, _) =
            self.move_player(*pos, *scale, na::Vector2::<f32>::new(-1.0, 0.0), &mut r_toi);
        if leftgoingray < 1.0 {
            l = true;
        }

        let (rightgoingray, _, _) =
            self.move_player(*pos, *scale, na::Vector2::<f32>::new(1.0, 0.0), &mut l_toi);
        if (rightgoingray < 1.0) {
            r = true;
        }

        let leftdowngoingray = self.raycast(
            *pos + na::Vector2::<f32>::new(-scale.x, -scale.y),
            na::Vector2::<f32>::new(0.0, -1.0),
            1.0,
        );
        match leftdowngoingray {
            Some((x, y)) => {
                ld_toi = y;
                ld = true;
            }
            None => (),
        }

        let rightdowngoingray = self.raycast(
            *pos + na::Vector2::<f32>::new(scale.x, -scale.y),
            na::Vector2::<f32>::new(0.0, -1.0),
            1.0,
        );
        match rightdowngoingray {
            Some((x, y)) => {
                rd_toi = y;
                rd = true;
            }
            None => (),
        }

        let mut min_toi: f32 = 100.0;
        let mut r_normal = na::Vector2::<f32>::new(0.0, 0.0);
        let mut t_contact = na::Vector2::<f32>::new(0.0, 0.0);
        //   println!("RD : {:?},LD : {:?} , L : {:?} , R : {:?}",rd,ld,l,r);
        // println!("RD : {:?},LD : {:?} , L : {:?} , R : {:?}",rd_toi,ld_toi,l_toi,r_toi);
        if (ld && l) || (r && rd) {
            for testagainst in &self.m_bodies {
                let (toi, normal, contact) = cutec2_sys::cutec2::sweep_test(
                    &pos,
                    &(scale * 2.0),
                    &na::Vector2::<f32>::new(0.0, -10.0),
                    &cutec2_sys::cutec2::make_poly(
                        &testagainst.position.xy(),
                        &(testagainst.scale.xy() * 2.0),
                        testagainst.rotation,
                    ),
                );
                if toi < min_toi {
                    min_toi = toi;
                    r_normal = normal;
                    t_contact = contact;
                }
            }
        }
        //Could Be 0 for not on a slope or 1 if slope is to your right or -1 if slope to your left
        let mut slope_side = 0;
        if t_contact != na::Vector2::zeros() {
            if pos.x - t_contact.x < 0.0 {
                let mut othernorm = na::Vector2::<f32>::new(0.0, 0.0);
                match self.raycast(
                    *pos + na::Vector2::<f32>::new(scale.x, -scale.y),
                    na::Vector2::<f32>::new(1.0, 0.0),
                    7.0,
                ) {
                    Some((x, y)) => (othernorm = y),
                    None => (),
                }
                if (othernorm == rd_toi) {
                    slope_side = 1;
                }
                return (slope_side, rd_toi);
            } else {
                let mut othernorm = na::Vector2::<f32>::new(0.0, 0.0);
                match self.raycast(
                    *pos + na::Vector2::<f32>::new(-scale.x, -scale.y),
                    na::Vector2::<f32>::new(-1.0, 0.0),
                    7.0,
                ) {
                    Some((x, y)) => (othernorm = y),
                    None => (),
                }
                if (othernorm == ld_toi) {
                    slope_side = -1;
                }
                return (slope_side, ld_toi);
            }
        }
        (0, na::Vector2::<f32>::new(0.0, 0.0))
    }

    pub fn move_player(
        &self,
        pos: na::Vector2<f32>,
        scale: na::Vector2<f32>,
        vel: na::Vector2<f32>,
        normal: &mut na::Vector2<f32>,
    ) -> (
        f32,
        std::option::Option<(na::Vector2<f32>, na::Vector2<f32>, f32)>,
        std::option::Option<na::Vector2<f32>>,
    ) {
        let mut min_toi: f32 = 100.0;
        let mut r_normal = na::Vector2::<f32>::new(0.0, 0.0);
        let mut t_position = na::Vector2::<f32>::new(0.0, 0.0);
        let mut t_scale = na::Vector2::<f32>::new(0.0, 0.0);
        let mut t_rotation = 0.0;;
        let mut t_contact = na::Vector2::<f32>::new(0.0, 0.0);
        for testagainst in &self.m_bodies {
            let (toi, normal, contact) = cutec2_sys::cutec2::sweep_test(
                &pos,
                &(scale * 2.0),
                &vel,
                &cutec2_sys::cutec2::make_poly(
                    &testagainst.position.xy(),
                    &(testagainst.scale.xy() * 2.0),
                    testagainst.rotation,
                ),
            );
            if toi < min_toi {
                min_toi = toi;
                r_normal = normal;
                t_position = testagainst.position.xy();
                t_scale = testagainst.scale.xy();
                t_contact = contact;
                t_rotation = testagainst.rotation;
            }
        }
        *normal = r_normal;
        if t_position.x == 0.00 && t_position.y == 0.0 {
            return (min_toi, Some((t_position, t_scale, t_rotation)), None);
        } else {
            return (
                min_toi,
                Some((t_position, t_scale, t_rotation)),
                Some(t_contact),
            );
        }
    }
    pub fn perform_ngs(&self, pos: na::Vector2<f32>, scale: na::Vector2<f32>) -> na::Vector2<f32> {
        let max_iters = 100;
        let mut iters = 0;
        let skin_factor: f32 = 0.02;
        let mut position = pos;

        let corrective_factor = 0.03;
        let mut running = true;

        while iters < max_iters && running {
            let mut hitsomething = 0;
            iters += 1;
            for clldr in &self.m_bodies {
                let (depth, normal) = cutec2_sys::cutec2::check_collision(
                    &position,
                    &(scale * 2.0 + na::Vector2::<f32>::new(skin_factor, skin_factor) * 2.0),
                    &cutec2_sys::cutec2::make_poly(
                        &clldr.position.xy(),
                        &(clldr.scale.xy() * 2.0),
                        clldr.rotation,
                    ),
                );
                if depth != 0.0 {
                    position += normal.xy() * corrective_factor;
                    hitsomething = 1;
                }
            }

            if hitsomething == 0 {
                running = false;
            }
        }
        position
    }
}
//that's pure ass shit but it's just a hack to get around the problem of not being able to have a nested mutating loop in rust
impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, components::Collider>);
    fn run(&mut self, mut collider: Self::SystemData) {
        let iterator = (&mut collider).join();

        for clldr in iterator {
            if clldr.body_handle == -1 {
                self.m_bodies.push(PhysicsBody {
                    position: clldr.position,
                    scale: clldr.scale,
                    vel: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
                    rotation: clldr.rotation,
                });
                clldr.body_handle = (self.m_bodies.len()) as i16 - 1;
            } else {
                clldr.position = self
                    .m_bodies
                    .get(clldr.body_handle as usize)
                    .unwrap()
                    .position;
                self.m_bodies
                    .get_mut(clldr.body_handle as usize)
                    .unwrap()
                    .vel = clldr.vel;
            }
        }
    }
}
