use super::model::{Boid, Velocity};

pub fn brain(boid: &Boid) -> Option<Velocity> {
    let new_velocity = boid.velocity.turn(0.01);
    Some(new_velocity)
}
