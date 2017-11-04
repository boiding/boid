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
use boid::{BoidConfig, Boid};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    boids: Vec<Boid>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        self.boids.iter_mut().for_each(|b| b.clip(args.width, args.height));
        let mut boids: Vec<&Boid> = self.boids.iter().map(|b| b.clone()).collect();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for boid in boids.iter_mut() {
                let rotation = boid.heading;
                let (x, y) = (boid.x, boid.y);


                let transform = c.transform.trans(x, y)
                    .rot_rad(rotation)
                    .trans(-25.0, -25.0);

                rectangle(RED, square, transform, gl);
            }
        });
    }

    fn update(&mut self, _: &UpdateArgs) {
        for boid in self.boids.iter_mut() {
            boid.update()
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "boiding",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let boid_config = BoidConfig::new(100f64, 100f64, 5f64);
    let boids: Vec<Boid> = (1..10)
        .map(|_| boid_config.random(&mut rng) )
        .collect();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        boids,
    };

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
