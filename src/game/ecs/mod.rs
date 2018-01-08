extern crate specs;
extern crate sdl2;

mod components;
mod systems;

use self::components::*;
use self::systems::*;
use self::specs::{World, DispatcherBuilder, Dispatcher};

pub struct DeltaTime(pub u64);

pub type Command = fn(specs::Entity);

pub fn build_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Display>();
    world.add_resource(DeltaTime(0));
    world
}

pub fn build_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
            .add(UpdatePosition, "update_position", &[])
            .build()
}

pub fn display(world: &World, canvas: &mut sdl2::render::WindowCanvas) {
    use self::specs::RunNow;
    let mut display = DisplaySystem::new(canvas);
    display.run_now(&world.res);
}

pub fn create_player(world: &mut World) -> specs::Entity {
    world.create_entity().with(Position{x: 10, y: 10})
                         .with(Display{rec: sdl2::rect::Rect::new(0, 0, 10, 10), color: sdl2::pixels::Color::RGB(255, 0, 0)})
                         .build()
}
