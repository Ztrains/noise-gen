
use std::f32::consts::{PI, SQRT_2};

use rand::Rng;

use crate::vector_2d::Vector2D;
use crate::SIZE;


// permutation table for lookup of pseudorandom constant vectors
pub fn create_permutation_table() -> [u32; SIZE*2] {
    
    let mut table: [u32; SIZE*2] = [0; SIZE*2];
    
    for index in 0..SIZE {
        table[index] = index as u32;
    }

    shuffle(&mut table);

    // need to 'wrap' the noise by doubling the shuffled permutation table
    for index in SIZE..SIZE*2 {
        table[index] = table[index - SIZE];
    }

    table
}

// the gradient table is not used in "improved" perlin noise
pub fn create_gradient_table() -> [Vector2D; SIZE] {
    
    let mut table: [Vector2D; SIZE] = [Vector2D{x: 0.0, y: 0.0}; SIZE];
    let mut rng = rand::thread_rng();

    for index in 0..SIZE {

        let random: f32 = rng.gen_range(0.0..(2.0 * PI));
        let mut gradient = Vector2D { x: random.cos(), y: random.sin() };

        // set vector length to sqrt(2) to get end noise values of [-1,1]
        let gradient_len = (gradient.x.powi(2) + gradient.y.powi(2)).sqrt();
        let desired_len = SQRT_2;

        gradient.x *= desired_len / gradient_len;
        gradient.y *= desired_len / gradient_len;

        table[index] = gradient;
    }

    table
}

pub fn get_constant_vector(perm: u32) -> Vector2D {
    let val = perm & 3;
    match val {
        0 => Vector2D { x: 1.0, y: 1.0 },
        1 => Vector2D { x: -1.0, y: 1.0 },
        2 => Vector2D { x: 1.0, y: -1.0 },
        _ => Vector2D { x: -1.0, y: -1.0 },
    }
}

// use Fisher-Yates algorithm to shuffle array in place
fn shuffle(arr: &mut [u32]) {

    let mut rng = rand::thread_rng();
    let mut i = arr.len() / 2 - 1;
    let mut rand_index;

    while i > 0 {
        rand_index = rng.gen_range(0..i+1);

        // swap arr[i] with arr[randIndex]
        arr.swap(i, rand_index);

        i -= 1;
    }
}
