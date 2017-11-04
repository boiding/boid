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
use std::iter::Iterator;
use boid::{App, BoidConfig, Boid};

fn main() {
    let mut rng = thread_rng();
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "boiding",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let boid_config = BoidConfig::new(100f64, 100f64, 5f64);
    let boids = boid_config.group_of(20, &mut rng);

    let mut app = App::new(GlGraphics::new(opengl), boids);

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
