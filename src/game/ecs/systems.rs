use super::specs::{Fetch, System, WriteStorage, ReadStorage, Join};
extern crate std;

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

pub struct DisplaySystem<'a> {
    canvas: &'a mut super::sdl2::render::WindowCanvas
}

impl<'a> DisplaySystem<'a> {
    pub fn new(canvas: &'a mut super::sdl2::render::WindowCanvas) -> DisplaySystem<'a> {
        DisplaySystem {
            canvas: canvas
        }
    }
}

impl<'a> System<'a> for DisplaySystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Display>);

    fn run(&mut self, (pos, display): Self::SystemData) {
        for (pos, display) in (&pos, &display).join() {
            let mut rect = display.rec.clone();
            rect.set_x(pos.x);
            rect.set_y(pos.y);
            let old = self.canvas.draw_color();
            self.canvas.set_draw_color(display.color.clone());
            let _ = self.canvas.fill_rect(rect);
            self.canvas.set_draw_color(old);
        }
    }
}