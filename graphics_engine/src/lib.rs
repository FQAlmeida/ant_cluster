extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;
use piston::window::WindowSettings;
use piston::{EventSettings, Events, RenderEvent, UpdateArgs, UpdateEvent};

pub struct App {
    gl: GlGraphics,
    pub window_handle: Window,
    scene_height: usize,
    scene_width: usize,
    state: Vec<Object>,
    pub get_state: Box<dyn FnMut() -> Vec<Object> + 'static>,
}

pub struct Object {
    pos: (usize, usize),
    color: [f32; 4],
}

impl Object {
    pub fn create(x: usize, y: usize, color: [f32; 4]) -> Object{
        Object{
            pos: (x, y),
            color
        }
    }
}

pub const WHITE: graphics::types::Color = [1.0, 1.0, 1.0, 1.0];
pub const RED: graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
pub const BLACK: graphics::types::Color = [0.0, 0.0, 0.0, 1.0];

impl App {
    pub fn create(
        title: &'static str,
        scene_height: usize,
        scene_width: usize,
        handle_get_state: Box<dyn FnMut() -> Vec<Object> + 'static>,
    ) -> Self {
        let opengl = OpenGL::V3_2;

        // Create a Glutin window.
        let window: Window = WindowSettings::new(title, [400, 400])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let empty_state: Vec<Object> = vec![];
        App {
            gl: GlGraphics::new(opengl),
            window_handle: window,
            scene_height,
            scene_width,
            state: empty_state,
            get_state: handle_get_state,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let (win_width, win_height) = (args.window_size[0], args.window_size[1]);
        let scene_height = self.scene_height;
        let scene_width = self.scene_width;
        let rect_width = win_width / scene_width as f64;
        let rect_height = win_height / scene_height as f64;

        let objects = self.state.iter();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BLACK, gl);

            for object in objects {
                let (xn, yn) = object.pos;
                let color = object.color;
                let x = rect_width * yn as f64;
                let xf = rect_width * (yn + 1) as f64;
                let y = rect_height * xn as f64;
                let yf = rect_height * (xn + 1) as f64;
                let rect: graphics::types::Rectangle = [x, y, xf, yf];
                let t = c.transform;
                graphics::rectangle(color, rect, t, gl);
            }
        });
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        self.state = (self.get_state)();
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window_handle) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }
}
