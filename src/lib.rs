extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod model;
mod app;

pub use model::{BoidConfig, Boid};
pub use app::App;
