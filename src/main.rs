use rand::{RngExt, rngs::ThreadRng};
use std::f64::consts::TAU;

fn main() {
    //il faut générer un vecteur de aléatoire du cercle trigonometrique
    println!("Hello, world!");
}

fn vecteur_gradient() -> (f64, f64) {
    let mut rng: ThreadRng = rand::rng();
    let theta: f64 = rng.random_range(0.0..TAU);
    let vector: (f64, f64) = (theta.cos(), theta.sin());
    vector
}

fn smooth_step(w: f64) -> f64 {
    if w <= 0.0 {
        0.0
    } else if w >= 1.0 {
        1.0
    } else {
        w * w * (3.0 - 2.0 * w)
    }
}

fn dot_grid_gradient(ix: i32, iy: i32, x: f64, y: f64) -> f64 {
    let dx: f64 = x - f64::from(ix);
    let dy: f64 = y - f64::from(iy);
    dx * vecteur_gradient().0 + dy * vecteur_gradient().1
}
