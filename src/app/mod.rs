use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::GlGraphics;
use graphics::{rectangle, clear, Transformed};
use super::model::Boid;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics,
    boids: Vec<Boid>,
}

impl App {
    pub fn new(gl: GlGraphics, boids: Vec<Boid>) -> App {
        App { gl, boids }
    }

    pub fn render(&mut self, args: &RenderArgs) {

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

    pub fn update(&mut self, _: &UpdateArgs) {
        for boid in self.boids.iter_mut() {
            boid.update()
        }
    }
}

