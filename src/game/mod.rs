extern crate sdl2;
extern crate specs;

mod utils;
mod ecs;
use self::utils::timer;

const MS_PER_FRAME: u64 = ((1. / 60.) * 1000.) as u64;

pub struct Game<'a, 'b> {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas,
    is_open: bool,
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'b>
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Game<'a, 'b> {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();

        let window = match video_ctx.window("test", 640, 480).position_centered().opengl().build() {
            Ok(window) => window,
            Err(err) => panic!("Failed to create window: {}", err)
        };
        
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

        Game {
            sdl_context: ctx,
            canvas: canvas,
            is_open: true,
            world: ecs::build_world(),
            dispatcher: ecs::build_dispatcher()
        }
    }

    pub fn run(&mut self) {
        let mut timer = timer::Timer::new();
        let mut time_since_last_update = 0;
        while self.is_open {
            time_since_last_update += timer.restart();
            self.process_events();
            while time_since_last_update > MS_PER_FRAME {
                time_since_last_update -= MS_PER_FRAME;
                self.process_events();
                self.update(MS_PER_FRAME);
            }
            self.render();
        }
    }

    fn process_events(&mut self) {
        use self::sdl2::event::Event;
        use self::sdl2::keyboard::Keycode;
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

    fn handle_player_input(&mut self, key: sdl2::keyboard::Keycode, _pressed: bool) {
        use self::sdl2::keyboard::Keycode;
        match key {
            Keycode::Z => {},
            Keycode::Q => {},
            Keycode::S => {},
            Keycode::D => {},
            _ => {}
        }
    }

    fn update(&mut self, delta: u64) {
        {
            let mut d = self.world.write_resource::<ecs::DeltaTime>();
            *d = ecs::DeltaTime(delta);
        }
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();
    }

    fn render(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }
}