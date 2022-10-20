extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;
use piston::window::WindowSettings;
use piston::{Event, EventSettings, Events, RenderEvent, UpdateArgs, UpdateEvent};

pub struct App {
    gl: GlGraphics,
    pub window_handle: Window,
    scene_height: usize,
    scene_width: usize,
    state: Vec<Object>,
}

pub struct Object {
    pos: (usize, usize),
    color: [f32; 4],
}

pub struct EventsBridge {
    events: Events,
}

pub struct EventBridge {
    event: Event,
}

impl EventBridge {
    fn create(event: Event) -> Self {
        Self { event }
    }
    pub fn render_args(&self) -> Option<RenderArgs> {
        self.event.render_args()
    }
    pub fn update_args(&self) -> Option<UpdateArgs> {
        self.event.update_args()
    }
}

impl EventsBridge {
    pub fn create() -> Self {
        Self {
            events: EventsBridge::create_event_handler(),
        }
    }
    fn create_event_handler() -> Events {
        Events::new(EventSettings::new())
    }
    pub fn next(&mut self, window: &mut Window) -> Option<EventBridge> {
        let event = self.events.next(window);
        return match event {
            Some(e) => Some(EventBridge::create(e)),
            None => None,
        };
    }
}

impl Object {
    pub fn create(x: usize, y: usize, color: [f32; 4]) -> Object {
        Object { pos: (x, y), color }
    }
}

pub const WHITE: graphics::types::Color = [1.0, 1.0, 1.0, 1.0];
pub const RED: graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
pub const BLACK: graphics::types::Color = [0.0, 0.0, 0.0, 1.0];

impl App {
    pub fn create(title: &'static str, scene_height: usize, scene_width: usize) -> Self {
        let opengl = OpenGL::V3_2;

        // Create a Glutin window.
        let window: Window = WindowSettings::new(title, [1024, 687])
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
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let (win_width, win_height) = (args.window_size[0], args.window_size[1]);
        let scene_height = self.scene_height + 1;
        let scene_width = self.scene_width + 1;
        let rect_width = win_width / scene_width as f64;
        let rect_height = win_height / scene_height as f64;
        // println!(
        //     "{} {} {} {} {} {}",
        //     win_height, win_width, scene_height, scene_width, rect_height, rect_width
        // );

        let objects = self.state.iter();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BLACK, gl);
            let t = c.transform;
            // println!("{:?}", t);

            for object in objects {
                let (i, j) = object.pos;
                let color = object.color;
                let x = rect_width * i as f64;
                let y = rect_height * j as f64;
                let rectangle: graphics::types::Rectangle = [x, y, rect_width, rect_height];
                graphics::rectangle(color, rectangle, t, gl);
            }
        });
    }

    pub fn update(&mut self, _: &UpdateArgs, new_state: Vec<Object>) {
        self.state = new_state;
    }
}
