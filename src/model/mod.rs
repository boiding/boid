use std::f64::consts::PI;
use rand::Rng;

pub struct Boid {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    speed: f64,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Boid {
        Boid { x, y, heading: 0f64, speed: 3f64 }
    }

    pub fn update(&mut self) {
        let dx = self.speed * self.heading.cos();
        let dy = self.speed * self.heading.sin();

        self.x += dx;
        self.y += dy;
    }

    pub fn clip(&mut self, width: u32, height: u32) {
        let width = width as f64;
        let height = height as f64;

        let factor = (self.x / width).floor();
        self.x -= factor * width;

        let factor = (self.y / height).floor();
        self.y -= factor * height;
    }
}

pub struct BoidConfig {
    max_x: f64,
    max_y: f64,
    max_speed: f64,
}

impl BoidConfig {
    pub fn new(max_x: f64, max_y: f64, max_speed: f64) -> BoidConfig {
        BoidConfig { max_x, max_y, max_speed }
    }

    pub fn random<R>(&self, rng: &mut R) -> Boid where R: Rng {
        let x = self.max_x * rng.next_f64();
        let y = self.max_y * rng.next_f64();
        let heading = 2f64 * PI * rng.next_f64();
        let speed = self.max_speed * rng.next_f64();

        Boid { x, y, heading, speed }
    }
}
