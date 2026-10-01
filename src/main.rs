use image::{GrayImage, Luma};
use rand::RngExt;
use std::f64::consts::TAU;

fn main() {
    generate_image(512, 512, 50);
}

//function to generate a gradient map
fn gradient_map(x: i32, y: i32) -> Vec<Vec<(f64, f64)>> {
    let mut grad_map = vec![];
    let mut i = 0;
    while i < x {
        let mut row = vec![];
        let mut j = 0;
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
    let mut rng = rand::rng();
    let theta = rng.random_range(0.0..TAU);
    (theta.cos(), theta.sin())
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
fn dot_grid_gradient(ix: i32, iy: i32, x: f64, y: f64, gv: &[Vec<(f64, f64)>]) -> f64 {
    let dx = x - ix as f64;
    let dy = y - iy as f64;
    dx * gv[iy as usize][ix as usize].0 + dy * gv[iy as usize][ix as usize].1
}

//function to interpolate and get a value between 2 points based on the distance from each (weight)
fn interpolate(a0: f64, a1: f64, w: f64) -> f64 {
    a0 + (a1 - a0) * smooth_step(w)
}

//function converting the perlin noise value to a grayscale value to use in the image creation
fn convert(value: f64) -> u8 {
    let normalized = ((value + 1.0) / 2.0) * 255.0;
    normalized.round() as u8
}

//finally saves the image in the Images folder in the project
fn save_image(image: GrayImage) {
    image
        .save("Images/perlin.png")
        .expect("Failed to save image");
}

//function calculating the value of the perlin noise at the coordinates given
fn perlin(x: f64, y: f64, gv: &[Vec<(f64, f64)>]) -> f64 {
    //calculate the coords of the corners of the cell where the point is in
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let y0 = y.floor() as i32;
    let y1 = y0 + 1;

    //interpolation weight
    let sx = x - x0 as f64;
    let sy = y - y0 as f64;

    //interpolation between all points
    let n0 = dot_grid_gradient(x0, y0, x, y, gv);
    let n1 = dot_grid_gradient(x1, y0, x, y, gv);
    let ix0 = interpolate(n0, n1, sx);

    let n0 = dot_grid_gradient(x0, y1, x, y, gv);
    let n1 = dot_grid_gradient(x1, y1, x, y, gv);
    let ix1 = interpolate(n0, n1, sx);

    interpolate(ix0, ix1, sy)
}

//function to generate the image as a PNG, gets an height and a widht and a scale
fn generate_image(height: i32, width: i32, scale: i32) {
    //it needs to first calculate the width and height of the gradient map to cover all "corners" of
    //cells
    let gradient_width = width as f64 / scale as f64;
    let gradient_width = gradient_width.ceil() + 1.0;
    let gradient_height = height as f64 / scale as f64;
    let gradient_height = gradient_height.ceil() + 1.0;
    let gradient_map = gradient_map(gradient_width as i32, gradient_height as i32);

    let mut image = GrayImage::new(width as u32, height as u32);

    //adjust the scale of the perlin cells to the scale of the image : more cells = more details and
    //the "scale" parameter is lower
    for y in 0..height {
        for x in 0..width {
            let pixel_x = x as f64 / scale as f64;
            let pixel_y = y as f64 / scale as f64;

            //calculate the perlin noise value for the current pixel
            let pixel_value = perlin(pixel_x, pixel_y, &gradient_map);

            //convert the perlin noise value into a gray value
            let pixel_value = convert(pixel_value);

            //for every pixel write the calculated gray value
            image.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }
    save_image(image);
}
