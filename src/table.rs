use std::f32::consts::{PI, SQRT_2};

use rand::Rng;

use crate::vector_2d::Vector2D;
use crate::SIZE;


pub fn create_permutation_table() -> [u32; SIZE*2] {
    let mut table: [u32; SIZE*2] = [0; SIZE*2];
    for index in 0..SIZE {
        table[index] = index as u32;
    }

    //println!("permTable before shuffle: {:?}", table);

    shuffle(&mut table);

    //println!("permTable after shuffle: {:?}", table);

    // need to 'wrap' the noise by doubling the shuffled permutation table
    for index in SIZE..SIZE*2 {
        table[index] = table[index - SIZE];
    }

    //println!("permTable after doubling: {:?}", table);

    table
}

// another option to consider for generating random gradients:
// roll a single number between 0 and 2π. Call it θ. Your vector is (cos θ, sin θ). 
pub fn create_gradient_table() -> [Vector2D; SIZE] {
    let mut table: [Vector2D; SIZE] = [Vector2D{x: 0.0, y: 0.0}; SIZE];
    let mut rng = rand::thread_rng();

    for index in 0..SIZE {

        let random: f32 = rng.gen_range(0.0..(2.0 * PI));

        let mut gradient = Vector2D { x: random.cos(), y: random.sin() };

        /* rng.gen::<f32>() generates a float between [0,1)
        which is then multiplied by 2 and subtracts 1 to convert the range to [-1.0, 1.0) */
        /*let mut gradient = Vector2D {
            x: (2.0 * rng.gen::<f32>()) - 1.0, 
            y: (2.0 * rng.gen::<f32>()) - 1.0
        };*/

        // set vector length to sqrt(2) to get end noise values of [-1,1]
        let gradient_len = (gradient.x.powi(2) + gradient.y.powi(2)).sqrt();
        let desired_len = SQRT_2;

        gradient.x *= desired_len / gradient_len;
        gradient.y *= desired_len / gradient_len;

        /*gradient.x /= gradient_len;
        gradient.y /= gradient_len;*/


        table[index] = gradient;
    }

    //println!("gradientTable: {:?}", table);

    table
}

fn shuffle(arr: &mut [u32]) {
    // use Fisher-Yates algorithm to shuffle array in place
    // (1..arr.len()).rev() means to start from end of array, ignoring first element
    /*for index in (1..arr.len()).rev() {
        
    }*/

    let mut rng = rand::thread_rng();
    let mut i = arr.len() / 2 - 1;
    let mut rand_index;

    while i > 0 {
        rand_index = rng.gen_range(0..i+1);

        //println!("before swap for i={}:", i);
        //println!("arr[i]={}, arr[randIndex]={}", arr[i], arr[randIndex]);

        // swap arr[i] with arr[randIndex]
        arr.swap(i, rand_index);

        //println!("after swap:");
        //println!("arr[i]={}, arr[randIndex]={}", arr[i], arr[randIndex]);

        i -= 1;
    }
}
