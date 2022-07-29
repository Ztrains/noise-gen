// after distance vector edits

pub mod vector_2d;
pub mod table;

pub const SIZE: usize = 256;

use crate::vector_2d::Vector2D;


pub fn noise_2d(x: f32, y: f32, perm_table: [u32; SIZE*2], grad_table: [Vector2D; SIZE]) -> f32 {

    //let permTable: [u32; SIZE*2] = createPermutationTable();
    //let gradTable: [Vector2D; SIZE] = createGradientTable();

    //get points x and y mod SIZE for permutation table lookup
    

    // calculate grid points
    /*let x0 = x.floor() as usize % SIZE;
    let x1 = x0 + 1;
    let y0 = y.floor() as usize % SIZE;
    let y1 = y0 + 1;*/

    // get x,y for permutation table lookup
    let xp = x.floor() as usize % SIZE;
    let yp = y.floor() as usize % SIZE;

    // calculate interpolation weights
    let xf = x - x.floor();
    let yf = y - y.floor();


    // calculate vectors from each grid corner to point (x,y)
    let vec_bot_left: Vector2D = Vector2D {
        x: xf,
        y: yf
    };
    let vec_bot_right: Vector2D = Vector2D {
        x: xf - 1.0,
        y: yf
    };
    let vec_top_left: Vector2D = Vector2D {
        x: xf,
        y: yf - 1.0
    };
    let vec_top_right: Vector2D = Vector2D {
        x: xf - 1.0,
        y: yf - 1.0
    };


    // get pseudorandom value (hash) from permutation table for each grid corner
    let val_bot_left: u32 = perm_table[perm_table[xp] as usize + yp];
    let val_bot_right: u32 = perm_table[perm_table[xp+1] as usize + yp];
    let val_top_left: u32 = perm_table[perm_table[xp] as usize + yp+1];
    let val_top_right: u32 = perm_table[perm_table[xp+1] as usize + yp+1];

    // get associated gradient for each hash
    let grad_bot_left: Vector2D = grad_table[val_bot_left as usize];
    let grad_bot_right: Vector2D = grad_table[val_bot_right as usize];
    let grad_top_left: Vector2D = grad_table[val_top_left as usize];
    let grad_top_right: Vector2D = grad_table[val_top_right as usize];

    /*let grad_bot_left: Vector2D = get_constant_vector(val_bot_left);
    let grad_bot_right: Vector2D = get_constant_vector(val_bot_right);
    let grad_top_left: Vector2D = get_constant_vector(val_top_left);
    let grad_top_right: Vector2D = get_constant_vector(val_top_right);*/


    // calculate dot product of gradient and vector for each grid corner
    let dot_bot_left = vec_bot_left.dot(grad_bot_left);
    let dot_bot_right = vec_bot_right.dot(grad_bot_right);
    let dot_top_left = vec_top_left.dot(grad_top_left);
    let dot_top_right = vec_top_right.dot(grad_top_right);

    // apply fade to weights
    let fade_x = fade(xf);
    let fade_y = fade(yf);

    //println!("normal dx,dy: {},{}", dx, dy);
    //println!("faded dx,dy: {},{}", fade_x, fade_y);

    // perform linear interpolation for left and right sides of grid
    let left_lerp = lerp(fade_y, dot_bot_left, dot_top_left);
    let right_lerp = lerp(fade_y, dot_bot_right, dot_top_right);

    // perform final linear interpolation to get end value
    lerp(fade_x, left_lerp, right_lerp)

}

// fade function for smoother interpolation (same as original fade from Ken Perlin)
fn fade(t: f32) -> f32 {
    6.0*t*t*t*t*t - 15.0*t*t*t*t + 10.0*t*t*t
}

// linear interpolation function
pub fn lerp(weight: f32, dot1: f32, dot2: f32) -> f32 {

    // regular linear interpolation
    //weight * (dot2 - dot1) + dot1

    // smoothstep interpolation
    (dot2 - dot1) * (3.0 - weight * 2.0) * weight * weight + dot1

    // smootherstep interpolation
    //(dot2 - dot1) * ((weight * (weight * 6.0 - 15.0) + 10.0) * weight * weight * weight) + dot1


}