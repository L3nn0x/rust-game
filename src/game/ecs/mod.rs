extern crate specs;

mod components;
mod systems;

use self::components::*;
use self::systems::*;
use self::specs::{World, DispatcherBuilder, Dispatcher};

pub struct DeltaTime(pub u64);

pub fn build_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.add_resource(DeltaTime(0));
    world
}

pub fn build_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
            .add(UpdatePosition, "update_position", &[])
            .build()
}