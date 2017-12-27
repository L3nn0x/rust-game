extern crate sdl2;

mod vec3;
mod timer;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;

const TIME_PER_FRAME: u64 = 16;

struct RectShape {
    rect: sdl2::rect::Rect,
    color: Color,
    up: bool,
    down: bool,
    left: bool,
    right: bool
}

impl RectShape {
    fn display(&self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        let color = canvas.draw_color();
        canvas.set_draw_color(self.color.clone());
        canvas.fill_rect(self.rect.clone())?;
        canvas.set_draw_color(color);
        Ok(())
    }

    fn movement(&mut self, movement: vec3::Vec3) {
        let x = self.rect.x();
        let y = self.rect.y();
        self.rect.set_x(x + movement.x as i32);
        self.rect.set_y(y + movement.y as i32);
    }
}

pub struct Game {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas,
    is_open: bool,
    player: RectShape
}

impl Game {
    pub fn new() -> Game {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();

        let window = match video_ctx.window("test", 640, 480).position_centered().opengl().build() {
            Ok(window) => window,
            Err(err) => panic!("Failed to create window: {}", err)
        };
        
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        Game {
            sdl_context: ctx,
            canvas: canvas,
            is_open: true,
            player: RectShape {
                rect: sdl2::rect::Rect::new(10, 10, 10, 10),
                color: Color::RGB(255, 0, 0),
                up: false,
                down: false,
                left: false,
                right: false
            }
        }
    }

    pub fn run(&mut self) {
        let mut timer = timer::Timer::new();
        let mut time_since_last_update = 0;
        while self.is_open {
            time_since_last_update += timer.restart();
            self.process_events();
            while time_since_last_update > TIME_PER_FRAME {
                time_since_last_update -= TIME_PER_FRAME;
                self.process_events();
                self.update(TIME_PER_FRAME);
            }
            self.render();
        }
    }

    fn process_events(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => self.is_open = false,
                Event::KeyDown { keycode: Some(x), ..} => self.handle_player_input(x, true),
                Event::KeyUp { keycode: Some(x), ..} => self.handle_player_input(x, false),
                _ => {}
            }
        }
    }

    fn handle_player_input(&mut self, key: sdl2::keyboard::Keycode, pressed: bool) {
        use self::sdl2::keyboard::Keycode;
        match key {
            Keycode::Z => self.player.up = pressed,
            Keycode::Q => self.player.left = pressed,
            Keycode::S => self.player.down = pressed,
            Keycode::D => self.player.right = pressed,
            _ => {}
        }
    }

    fn update(&mut self, delta: u64) {
        let mut movement = vec3::Vec3::default();
        movement.y += (self.player.down as i32 - self.player.up as i32) as f64;
        movement.x += (self.player.right as i32 - self.player.left as i32) as f64;
        self.player.movement(delta as f64 * movement);
    }

    fn render(&mut self) {
        self.canvas.clear();
        let _ = self.player.display(&mut self.canvas);
        self.canvas.present();
    }
}