pub struct MovementSystem;
use super::components;
use specs::{Join, ReadStorage, System, WriteStorage};
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, components::Transform>,
        ReadStorage<'a, components::Movement>,
    );
    fn run(&mut self, (mut transform, movement): Self::SystemData) {
        for (tran, mov) in (&mut transform, &movement).join() {
            (*tran).position += (*mov).velocity;
        }
    }
}
