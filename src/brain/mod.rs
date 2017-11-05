use super::model::Boid;

pub fn brain(boid: &Boid) -> Option<(f64, f64)> {
    Some((boid.heading + 0.01, boid.speed))
}
