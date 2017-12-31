use super::specs::{Fetch, System, WriteStorage, ReadStorage, Join};

use super::components::*;

pub struct UpdatePosition;

impl<'a> System<'a> for UpdatePosition {
    type SystemData = (Fetch<'a, super::DeltaTime>, WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        let delta = delta.0;
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.x * delta as i32;
            pos.y += vel.y * delta as i32;
        }
    }
}
