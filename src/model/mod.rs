use std::f64::consts::PI;
use rand::Rng;

pub mod velocity;

pub use self::velocity::Velocity;

#[derive(Debug, PartialEq, Clone)]
pub struct Boid {
    pub x: f64,
    pub y: f64,
    pub velocity: Velocity,
    pub target: Option<Velocity>,
    agility: f64
}

impl Boid {
    pub fn update(&mut self, dt: f64) {
        let (dx, dy) = self.velocity.delta();

        self.x += dx * dt;
        self.y += dy * dt;
    }

    pub fn clip(&mut self, width: u32, height: u32) {
        let width = width as f64;
        let height = height as f64;

        let factor = (self.x / width).floor();
        self.x -= factor * width;

        let factor = (self.y / height).floor();
        self.y -= factor * height;

        self.velocity.clip();
    }
}

pub struct BoidConfig {
    max_x: f64,
    max_y: f64,
    min_speed: f64,
    max_speed: f64,
    agility: f64,
    group_size: u16,
    size: f64,
}

impl BoidConfig {
    pub fn new(max_x: f64, max_y: f64, min_speed: f64, max_speed: f64, agility: f64, group_size: u16, size: f64) -> BoidConfig {
        BoidConfig { max_x, max_y, min_speed, max_speed, agility, group_size, size }
    }

    pub fn random<R>(&self, rng: &mut R) -> Boid where R: Rng {
        let x = self.max_x * rng.next_f64();
        let y = self.max_y * rng.next_f64();
        let heading = 2f64 * PI * rng.next_f64();
        let speed = self.min_speed + (self.max_speed - self.min_speed) * rng.next_f64();
        let velocity = Velocity::new(heading, speed);

        Boid { x, y, velocity, agility: self.agility, target: None }
    }

    pub fn group<R>(&self, rng: &mut R) -> Vec<Boid> where R: Rng {
        let boids: Vec<Boid> = (1..self.group_size)
            .map(|_| self.random(rng) )
            .collect();

        boids
    }

    pub fn size(&self) -> f64 {
        self.size
    }
}

pub struct BoidConfigFactory {
    max_x: f64,
    max_y: f64,
    min_speed: f64,
    max_speed: f64,
    agility: f64,
    group_size: u16,
    size: f64,
}

impl BoidConfigFactory {
    pub fn new() -> Self {
        BoidConfigFactory { max_x: 100f64, max_y: 100f64, min_speed: 200f64, max_speed: 300f64, agility: 0.1, group_size: 20, size: 10f64 }
    }

    pub fn with_max_x(mut self, max_x: f64) -> Self {
        self.max_x = max_x;

        self
    }

    pub fn with_max_y(mut self, max_y: f64) -> Self {
        self.max_y = max_y;

        self
    }

    pub fn with_min_speed(mut self, min_speed: f64) -> Self {
        self.min_speed = min_speed;

        self
    }

    pub fn with_max_speed(mut self, max_speed: f64) -> Self {
        self.max_speed = max_speed;

        self
    }

    pub fn with_agility(mut self, agility: f64) -> Self {
        self.agility = agility;

        self
    }

    pub fn with_group_size(mut self, group_size: u16) -> Self {
        self.group_size = group_size;

        self
    }

    pub fn with_size(mut self, size: f64) -> Self {
        self.size = size;

        self
    }

    pub fn build(self) -> BoidConfig {
        BoidConfig::new(self.max_x, self.max_y, self.min_speed, self.max_speed, self.agility, self.group_size, self.size)
    }
}
