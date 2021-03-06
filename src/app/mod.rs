use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::GlGraphics;
use graphics::{ellipse, clear, Transformed};
use graphics::ellipse::circle;
use rand::Rng;
use super::model::{BoidConfig, Boid};
use super::brain::brain;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics,
    config: BoidConfig,
    boids: Vec<Boid>,
}

impl App {
    pub fn new<R>(gl: GlGraphics, config: BoidConfig, rng: &mut R) -> App where R: Rng {
        let boids = config.group(rng);
        App { gl, config, boids  }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        let shape = circle(0.0, 0.0, self.config.size());
        let center = self.config.size() / 2.0;

        self.boids.iter_mut().for_each(|b| b.clip(args.width, args.height));
        let boids: Vec<Boid> = self.boids.iter().map(|b| b.clone()).collect();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for boid in boids {
                let rotation = boid.velocity.heading;
                let (x, y) = (boid.x, boid.y);


                let transform = c.transform.trans(x, y)
                    .rot_rad(rotation)
                    .trans(-center, -center);

                ellipse(RED, shape, transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for boid in self.boids.iter_mut() {
            boid.update(args.dt)
        }
        let mut clique: Vec<Boid> = vec!();
        for b in &self.boids {
            let clone = b.clone();
            clique.push(clone);
        }
        for boid in self.boids.iter_mut() {
            match brain(boid, &clique) {
                Some(velocity) => {
                    boid.velocity = velocity;
                },
                None => {
                    /* do nothing */
                }
            }
        }
    }
}
