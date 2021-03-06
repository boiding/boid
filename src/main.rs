extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate boid;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use rand::thread_rng;
use boid::{App, BoidConfigFactory};

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

fn main() {
    let mut rng = thread_rng();
    let boid_config = BoidConfigFactory::new()
        .with_max_x(100f64)
        .with_max_y(100f64)
        .with_min_speed(200f64)
        .with_max_speed(300f64)
        .with_group_size(20)
        .with_size(10f64)
        .build();

    let mut window: Window = WindowSettings::new(
            "boiding",
            [200, 200]
        )
        .opengl(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(OPENGL_VERSION), boid_config, &mut rng);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
