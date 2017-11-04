use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::GlGraphics;
use graphics::{ellipse, clear, Transformed};
use graphics::ellipse::circle;
use super::model::Boid;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct ShapeConfig {
    size: f64,
}

impl ShapeConfig {
    pub fn new(size: f64) -> ShapeConfig {
        ShapeConfig { size }
    }
}

pub struct App {
    gl: GlGraphics,
    shape_config: ShapeConfig,
    boids: Vec<Boid>,
}

impl App {
    pub fn new(gl: GlGraphics, shape_config: ShapeConfig, boids: Vec<Boid>) -> App {
        App { gl, shape_config, boids }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        let shape = circle(0.0, 0.0, self.shape_config.size);
        let center = self.shape_config.size / 2.0;

        self.boids.iter_mut().for_each(|b| b.clip(args.width, args.height));
        let boids: Vec<&Boid> = self.boids.iter().map(|b| b.clone()).collect();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for boid in boids {
                let rotation = boid.heading;
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
    }
}

