extern crate sdl2;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut timer = ctx.timer().unwrap();

    let mut window = match video_ctx.window("test", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err)
    };

    window.show();
    timer.delay(3000);
}
