use std::f64::consts::PI;

const TWO_PI: f64 = 2.0 * PI;

#[derive(Debug, PartialEq, Clone)]
pub struct Velocity {
    pub heading: f64,
    pub speed: f64,
}

impl Velocity {
    pub fn new(heading: f64, speed: f64) -> Velocity {
        Velocity { heading, speed }
    }

    pub fn delta(&self) -> (f64, f64) {
        (self.speed * self.heading.cos(), self.speed * self.heading.sin())
    }

    pub fn turn(&self, angle: f64) -> Velocity {
        Velocity::new(self.heading + angle, self.speed)
    }

    pub fn clip(&mut self) {
        let factor = (self.heading / TWO_PI).floor();
        self.heading -= factor * TWO_PI;
    }
}


