use super::model::Boid;
use super::model::velocity::Velocity;

pub fn brain(boid: &Boid, clique: &Vec<Boid>) -> Option<Velocity> {
    let new_velocity = boid.velocity.turn(0.01);
    Some(new_velocity)
}
