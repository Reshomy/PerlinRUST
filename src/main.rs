use rand::{RngExt, rngs::ThreadRng};
use std::f64::consts::TAU;

fn main() {
    let x = 4.6;
    let y = 3.9;
    let xtaille: i32 = 7;
    let ytaille: i32 = 7;
    let gradient_map: Vec<Vec<(f64, f64)>> = gradient_map(xtaille, ytaille);
    let get_value: f64 = perlin(x, y, &gradient_map);
    println!("{}", get_value);
}

//function to generate a gradient map
fn gradient_map(x: i32, y: i32) -> Vec<Vec<(f64, f64)>> {
    let mut grad_map: Vec<Vec<(f64, f64)>> = vec![];
    let mut i: i32 = 0;
    while i < x {
        let mut row: Vec<(f64, f64)> = vec![];
        let mut j: i32 = 0;
        while j < y {
            row.push(gradient_vector());
            j += 1;
        }
        i += 1;
        grad_map.push(row);
    }
    grad_map
}

//function to get a random gradient vector
fn gradient_vector() -> (f64, f64) {
    let mut rng: ThreadRng = rand::rng();
    let theta: f64 = rng.random_range(0.0..TAU);
    let vector: (f64, f64) = (theta.cos(), theta.sin());
    vector
}

//function to smooth out the result and get a good transition between the cells
fn smooth_step(w: f64) -> f64 {
    if w <= 0.0 {
        0.0
    } else if w >= 1.0 {
        1.0
    } else {
        w * w * (3.0 - 2.0 * w)
    }
}

//function to get the scalar product between the distance vector and the gradient vector
fn dot_grid_gradient(ix: i32, iy: i32, x: f64, y: f64, gv: &Vec<Vec<(f64, f64)>>) -> f64 {
    let dx: f64 = x - ix as f64;
    let dy: f64 = y - iy as f64;
    dx * gv[iy as usize][ix as usize].0 + dy * gv[iy as usize][ix as usize].1
}

//function to interpolate and get a value between 2 points based on the distance from each (weight)
fn interpolate(a0: f64, a1: f64, w: f64) -> f64 {
    a0 + (a1 - a0) * smooth_step(w)
}

fn perlin(x: f64, y: f64, gv: &Vec<Vec<(f64, f64)>>) -> f64 {
    //calculate the coords of the corners of the cell where the point is in
    let x0: i32 = x.floor() as i32;
    let x1: i32 = x0 + 1;
    let y0: i32 = y.floor() as i32;
    let y1: i32 = y0 + 1;

    //interpolation weight
    let sx: f64 = x - x0 as f64;
    let sy: f64 = y - y0 as f64;

    //interpolation between all points

    let n0: f64 = dot_grid_gradient(x0, y0, x, y, gv);
    let n1: f64 = dot_grid_gradient(x1, y0, x, y, gv);
    let ix0: f64 = interpolate(n0, n1, sx);

    let n0: f64 = dot_grid_gradient(x0, y1, x, y, gv);
    let n1: f64 = dot_grid_gradient(x1, y1, x, y, gv);
    let ix1: f64 = interpolate(n0, n1, sx);

    let value: f64 = interpolate(ix0, ix1, sy);
    value
}

fn generate_image(height: i32, width: i32, scale: i32) {
    let gradient_map: Vec<Vec<(f64, f64)>> = gradient_map(12, 12);
    for y in 0..height {
        for x in 0..width {
            let pixel_x: i32 = x / scale;
            let pixel_y: i32 = y / scale;
            perlin(pixel_x, pixel_y, &gradient_map)
        }
    }
}
