use std::f64::consts::PI;
use std::ops::{Mul, Add};

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

impl Mul<f64> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f64) -> Velocity {
        Velocity::new(self.heading, self.speed * rhs)
    }
}

impl Add for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Velocity) -> Velocity {
        let xl = self.speed * self.heading.cos();
        let yl = self.speed * self.heading.sin();

        let xr = rhs.speed * rhs.heading.cos();
        let yr = rhs.speed * rhs.heading.sin();

        let x = xl + xr;
        let y = yl + yr;

        let heading = y.atan2(x);
        let speed = x.hypot(y);

        Velocity::new(heading, speed)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn velocity_should_multiply_with_f64() {
        let left = Velocity::new(1.0, 3.0);
        let right = 2.0f64;

        let answer = left * right;

        assert_eq!(answer, Velocity::new(1.0, 6.0))
    }

    #[test]
    fn velocity_should_add_with_velocity() {
        let left = Velocity::new(1.0, 3.0);
        let right = Velocity::new(1.0, 3.0);

        let answer = left + right;

        assert!((answer.heading - 1.0).abs() < EPSILON);
        assert!((answer.speed - 6.0).abs() < EPSILON);
    }
}
