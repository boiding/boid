use std::f64::consts::PI;
use super::model::Boid;
use super::model::velocity::Velocity;

const AVERAGE_WEIGHT: f64 = 2.0;
const AVOIDANCE_WEIGHT: f64 = 3.0;
const TOTAL_WEIGHT: f64 = AVERAGE_WEIGHT + AVOIDANCE_WEIGHT;

pub fn brain(boid: &Boid, clique: &Vec<Boid>) -> Option<Velocity> {
    let average_velocity = average_velocity(&clique);
    let avoidance_velocity = avoid_closest(&boid, &clique);

    let mut velocity =
        average_velocity.clone() * AVERAGE_WEIGHT +
        avoidance_velocity.clone() * AVOIDANCE_WEIGHT;
    velocity = velocity * (1.0/TOTAL_WEIGHT);

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
    println!("{}", speed);

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
