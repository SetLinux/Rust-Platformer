use super::components;
use super::physics;
use specs::{Join, ReadStorage, System, WriteStorage};
use std::cell::RefCell;
use std::rc::Rc;
pub struct SweptMovement {
    pub physicssytem: Rc<RefCell<physics::PhysicsSystem>>,
}
impl SweptMovement {
    fn sweep_against_geo(
        &mut self,
        pos: &na::Vector2<f32>,
        scale: &na::Vector2<f32>,
        vel: &na::Vector2<f32>,
        t_pos: &na::Vector2<f32>,
        t_scale: &na::Vector2<f32>,
        t_rotation: f32,
    ) -> (f32, na::Vector2<f32>, na::Vector2<f32>) {
        (cutec2_sys::cutec2::sweep_test(
            pos,
            scale,
            vel,
            &cutec2_sys::cutec2::make_poly(t_pos, t_scale, t_rotation),
        ))
    }
    //Shouldn't be used for the final movement use proccess_movement  instead
    fn move_player(
        &mut self,
        pos: &na::Vector2<f32>,
        scale: &na::Vector2<f32>,
        vel: &na::Vector2<f32>,
    ) -> (
        na::Vector2<f32>,
        f32,
        na::Vector2<f32>,
        std::option::Option<(na::Vector2<f32>, na::Vector2<f32>, f32)>,
    ) {
        let point = self.physicssytem.try_borrow_mut().unwrap().perform_ngs(
            na::Vector2::<f32>::new(pos.x, pos.y),
            na::Vector2::<f32>::new(scale.x, scale.y),
        );

        let mut normal: na::Vector2<f32> = na::Vector2::<f32>::new(0.0, 0.0);
        let (mut toi, geo, _contact) = self.physicssytem.try_borrow_mut().unwrap().move_player(
            na::Vector2::<f32>::new(point.x, point.y),
            na::Vector2::<f32>::new(scale.x, scale.y),
            na::Vector2::<f32>::new(vel.x, vel.y),
            &mut normal,
        );
        if(na::Matrix::magnitude(&(vel * toi)) < 0.07f32) {
            println!("OK ZEROID IT");
            toi = 0.0;
        }
        //println!("MOVE PLAYER , :{:?} , {:?}",vel,na::Matrix::magnitude(&(vel * toi)));

        let poser: na::Vector2<f32> = point + vel * toi; //+ na::Vector3::<f32>::new(lvel.x,lvel.y,0.0);

        let ngsed = self.physicssytem.try_borrow_mut().unwrap().perform_ngs(
            na::Vector2::<f32>::new(poser.x, poser.y),
            na::Vector2::<f32>::new(scale.x, scale.y),
        );
        (ngsed, toi, normal, geo)
    }
    //Only use this for movement :)
    fn proccess_movement(
        &mut self,
        position: &na::Vector2<f32>,
        scale: &na::Vector2<f32>,
        velocity: &na::Vector2<f32>,
        jump: bool,
        iters: i16,
    ) -> (na::Vector2<f32>, na::Vector2<f32>,bool) {
        let (mut point, mut toi, mut normal, geo) = self.move_player(position, scale, velocity);
        println!("the velocity is : {:?},iters : {:?},normal : {:?},position : {:?},toi : {:?}",velocity,iters,normal,position,toi);
        if(na::Matrix::magnitude(&(point - position)) < 0.6) {
          //point = *position;
        }
        let mut refvelocity = *velocity;
            let mut climbed = false;
        if (iters < 17) {
            //here i am detecting that i am gonna reflect to a slope because then i wouldn't take the Y velocity in consideration
            //because i need to have the same speed wether i am having a downward velocity at the same time or not
            //(not realistic but makes the game feel more fluid which is all what matter )
            //the weird numbers is to account for some floating point-error
            //if normal.x.abs() > 0.01 && normal.x.abs() < 0.9999 && toi ;< 1.0 && normal.y > 0.0 {
            let mut slope_normal = normal;
            let (t_position, t_scale, t_rotation) = geo.unwrap();

            let (_toier, normaler, _contact) = self.sweep_against_geo(
                &point,
                &(scale * 2.0),
                &na::Vector2::<f32>::new(0.0, -1.0),
                &t_position,
                &(t_scale * 2.0),
                t_rotation,
            );
            //do that to avoid the shit happening when you aren't exactly on the slope
            if na::Matrix::magnitude(&normaler) > 0.1 && normaler.y >= 0.0 {
                slope_normal = normaler;
                //normal = normaler;
            }

            if slope_normal.x.abs() > 0.01 && slope_normal.x.abs() < 0.999 && toi < 1.0 && slope_normal.y > 0.0 && slope_normal.y.abs() < 0.99 {
                   normal = na::Vector2::<f32>::new(normal.y,-normal.x);

                  //calculating how much more movement is left so (1-toi) is the remaining toi assuming the max_toi is 1.0
                let dot_product = (velocity.x ) *(1f32 - toi);
                let (respoint, resvelocity,refclib) = self.proccess_movement(
                    &point,
                    scale,
                    &((velocity.x* (1f32 - toi))  * normal),
                    jump,

                    iters + 1,
                );
                climbed = true;
                point = respoint;
                refvelocity = resvelocity;;
            } else if toi < 1.0 && (normal.x.abs() > 0.99 || normal.y > 0.99) {
                println!("OK LET's REFLECT");
                let mut dot_product = 0.0;
                    normal = na::Vector2::<f32>::new(normal.y,normal.x);

                let (respoint, resvelocity,refclimb) = self.proccess_movement(
                    &point,
                    scale,
                    &((na::Matrix::dot(&normal,&(velocity*(1f32-toi))) * normal))
                    ,
                    jump,
                    iters + 1,
                );
                point = respoint;
                refvelocity = resvelocity;
                climbed = refclimb;
                
            }
             else if toi < 1.0 {
                //&& (normal.x.abs() > 0.99 || normal.y.abs() > 0.99) {
          
                let mut dot_product = 0.0;
                if (velocity.y < 0.0) || (velocity.y > 0.0 && normal.y == 0.0) {
                    dot_product = (velocity.x * normal.y + velocity.y * -normal.x) *(1f32 - toi);
                } else {
                    dot_product = velocity.x * normal.y * ( 1f32 - toi);
                }
                let (respoint, resvelocity,refclimb) = self.proccess_movement(
                    &point,
                    scale,
                    &na::Vector2::<f32>::new(dot_product * normal.y, dot_product * -normal.x),
                    jump,
                    iters + 1,
                );
                point = respoint;
                refvelocity = resvelocity;
            }
        }
        (point, refvelocity,climbed)
    }
}
impl<'a> System<'a> for SweptMovement {
    type SystemData = (
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Player>,
        WriteStorage<'a, components::Movement>,
    );

    fn run(&mut self, (mut trans, mut player, mut movement): Self::SystemData) {
        for (mut tran, mut plr, mvmnt) in (&mut trans, &mut player, &mut movement).join() {

            //let xmovedpos =  self.move_player(tran.position, tran.scale, na::Vector3::<f32>::new(plr.vel.x,0.0,0.0));
            //mvmnt.velocity =  self.move_player(xmovedpos, tran.scale, na::Vector3::<f32>::new(0.0,plr.vel.y,0.0)) - tran.position;
            let mut teset = na::Vector2::<f32>::new(0.0,0.0);
            let (mut toi, geo, _contact)  = self.physicssytem.try_borrow_mut().unwrap().move_player(tran.position.xy(),tran.scale.xy(),na::Vector2::<f32>::new(0.0,-2.0),&mut teset);
            if(toi < 0.7 && toi > 0.1) {
                tran.position = tran.position +  na::Vector3::<f32>::new(0.0,-2.0,0.0) * toi;
            }
            let (point, refvelocity,climbed) = self.proccess_movement(
                &tran.position.xy(),
                &tran.scale.xy(),
                &plr.vel.xy(),
                plr.jump,
                0,
            );
            if(climbed) {
            plr.vel = na::Vector3::<f32>::new(refvelocity.x, refvelocity.y, 0.0);
            }
            mvmnt.velocity = na::Vector3::<f32>::new(point.x, point.y, 0.0) - tran.position;
        }
    }
}

pub struct PlayerSystem {
    pub moveleft: bool,
    pub moveright: bool,
    pub moveup: bool,
    pub movedown: bool,
    pub physicssystem: Rc<RefCell<physics::PhysicsSystem>>,
}
impl PlayerSystem {}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, components::Transform>,
        WriteStorage<'a, components::Player>,
        ReadStorage<'a, components::Movement>,
    );

    fn run(&mut self, (mut trans, mut player, movement): Self::SystemData) {
        for (mut tran, ply, mov) in (&mut trans, &mut player, &movement).join() {
            let mut nigga: na::Vector2<f32> = na::Vector2::<f32>::new(0.0, 0.0);
            let mut velocity = ply.vel.xy();
            velocity.x = 0.0;
            let mut normaler = na::Vector2::<f32>::new(0.0,0.0);
            //   let (mut point, toi, mut normal, geo) = self.physicssystem.try_borrow_mut().unwrap().move_player(tran.position.xy(), tran.scale.xy(), na::Vector2::<f32>::new(0.0,-1.0),0.1);
            
            let (down_toi, _, _) = self.physicssystem.try_borrow_mut().unwrap().move_player(
                tran.position.xy(),
                tran.scale.xy(),
                na::Vector2::<f32>::new(0.0, -0.09),
                &mut normaler,
            );
            
            
            let (up_toi, _, _) = self.physicssystem.try_borrow_mut().unwrap().move_player(
                tran.position.xy(),
                tran.scale.xy(),
                na::Vector2::<f32>::new(0.0, 0.4),
                &mut nigga,
            );
            let (left_toi, _, _) = self.physicssystem.try_borrow_mut().unwrap().move_player(
                tran.position.xy(),
                tran.scale.xy(),
                na::Vector2::<f32>::new(-1.15, 0.00),
                &mut nigga,
            );
            let (right_toi, _, _) = self.physicssystem.try_borrow_mut().unwrap().move_player(
                tran.position.xy(),
                tran.scale.xy(),
                na::Vector2::<f32>::new(1.15, 0.00),
                &mut nigga,
            );
            let (side, slope_normal) = self
                .physicssystem
                .try_borrow_mut()
                .unwrap()
                .on_slope(&tran.position.xy(), &tran.scale.xy());
            if (down_toi < 1.0 || up_toi < 1.0 || side != 0) {
                //    println!("killed the Y Velocity {:?}",mov.velocity.xy());
                if (side != 0) {

                    velocity.y = 0.0;
                }
                if (down_toi < 1.0 && velocity.y < 0.0) {

                    velocity.y = 0.0;
                }
                if (up_toi < 1.0 && velocity.y > 0.0) {
                    velocity.y = 0.0;
                }
                //    tran.position = tran.position + na::Vector3::<f32>::new(0.0,-3.0,0.0) * toi;
            }
            velocity.y -= 1.5;
            ply.jump = false;

            //velocity -= na::Vector3::<f32>::new(0.0,8.0,0.0);
            if self.moveleft {
                if side == 1 {
                    let dot_product = -20.0;
                    velocity = na::Vector2::<f32>::new(
                        dot_product * slope_normal.y,
                        dot_product * -slope_normal.x,
                    );
                } else {
                    //if left_toi > 0.99 || side != 0{

                    velocity.x += -7.0;
                }
            }
            if self.moveright {
                if side == -1  && slope_normal.x.abs() < 0.99 && slope_normal.y.abs() < 0.99 && na::Matrix::magnitude(&slope_normal)> 0.01{
                    let dot_product = 20.0;
                    println!("I AM CLIMBING : {:?}",slope_normal);
                    velocity = na::Vector2::<f32>::new(
                        dot_product * slope_normal.y,
                        dot_product * -slope_normal.x,

                    );
                } else  {
                    velocity.x = 7.0;
//                    velocity.y = 1.5;
                }
            }
            if self.moveup && (down_toi < 1.0 || side != 0)&& up_toi > 0.99 {
                println!("JUMP : {:?}",normaler);
                velocity.y = 20.0;
            }
            if self.moveup {
                ply.jump = true;
            }
            if self.movedown {
                velocity.y -= 5.0;
            }
            self.moveleft = false;
            self.moveright = false;
            self.moveup = false;
            self.movedown = false;
            ply.vel = na::Vector3::<f32>::new(velocity.x, velocity.y, 0.0);
        }
    }
}
