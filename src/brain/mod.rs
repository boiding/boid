use super::model::Boid;
use super::model::velocity::Velocity;

pub fn brain(boid: &Boid, clique: &Vec<Boid>) -> Option<Velocity> {
    let n = (clique.len() + 1) as f64;
    let mut velocity = clique.iter()
        .map(|c| c.velocity.clone())
        .fold(boid.velocity.clone(), |acc, c| acc + c);
    velocity = velocity * (1f64/n);
    Some(velocity)
}
