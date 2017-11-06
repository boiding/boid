use std::f64::consts::PI;
use super::model::Boid;
use super::model::velocity::Velocity;

const AVERAGE_WEIGHT: f64 = 2.0;
const AVOIDANCE_WEIGHT: f64 = 3.0;
const SEEK_WEIGHT: f64 = 2.0;
const TOTAL_WEIGHT: f64 = AVERAGE_WEIGHT + AVOIDANCE_WEIGHT + SEEK_WEIGHT;

pub fn brain(boid: &Boid, clique: &Vec<Boid>) -> Option<Velocity> {
    let average_velocity = average_velocity(&clique);
    let avoidance_velocity = avoid_closest(&boid, &clique);
    let seek_velocity = seek_center(&boid, &clique);

    let mut velocity =
        average_velocity.clone() * AVERAGE_WEIGHT +
        avoidance_velocity.clone() * AVOIDANCE_WEIGHT +
        seek_velocity.clone() * SEEK_WEIGHT;
    velocity = velocity * (1.0/TOTAL_WEIGHT);

    if velocity.speed.abs() > 300.0 {
        velocity.speed = 300.0
    }

    Some(velocity)
}

fn average_velocity(clique: &Vec<Boid>) -> Velocity {
    let n = clique.len() as f64;
    let mut velocity = clique.iter()
        .map(|c| c.velocity.clone())
        .fold(Velocity::new(0.0, 0.0), |acc, c| acc + c);
    velocity = velocity * (1f64/n);

    velocity
}

fn avoid_closest(boid: &Boid, clique: &Vec<Boid>) -> Velocity {
    let closest = closest_boid(&boid, &clique);

    let x = closest.x - boid.x;
    let y = closest.y - boid.y;
    let heading = y.atan2(x) + PI;
    let distance = x.abs().hypot(y.abs());
    let speed = 300.0 * 5.0/distance;

    Velocity::new(heading, speed)
}

fn closest_boid(boid: &Boid, clique: &Vec<Boid>) -> Boid {
    let mut tuples: Vec<(Boid, f64)> = clique.iter()
        .map(|c| (c.clone(), distance(boid, c)))
        .filter(|t| t.1 > 0.0)
        .collect();
    tuples.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap() );

    tuples.first().unwrap().0.clone()
}

fn distance(u: &Boid, v: &Boid) -> f64 {
    (u.x - v.x).abs().hypot((u.y - v.y).abs())
}

fn seek_center(boid: &Boid, clique: &Vec<Boid>) -> Velocity {
    let n = clique.len() as f64;
    let (center_x, center_y) = clique.iter()
        .map(|c| (c.x, c.y))
        .fold((0f64, 0f64), |acc, t| (acc.0 + t.0, acc.1 + t.1));

    let dx = center_x - boid.x;
    let dy = center_y - boid.y;
    let heading = dy.atan2(dx);
    let speed = 100f64.max(10f64*dx.hypot(dy));

    let velocity = Velocity::new(heading, speed) * (1.0/n);
    velocity
}
