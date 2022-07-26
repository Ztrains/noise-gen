use std::{fs::File, io::{LineWriter, Write}, error::Error};

use image::{RgbImage, Rgb};
use rand::Rng;


const SIZE: usize = 256;
const MAPSIZE: usize = 400;
//const permTable: [u32; SIZE*2] = createPermutationTable();
//const gradTable: [Vector2D; SIZE] = createGradientTable();

#[derive(Clone, Copy, Debug)]
struct Vector2D {
    x: f32,
    y: f32
}

fn noise_2d(x: f32, y: f32, perm_table: [u32; SIZE*2], grad_table: [Vector2D; SIZE]) -> f32 {

    //let permTable: [u32; SIZE*2] = createPermutationTable();
    //let gradTable: [Vector2D; SIZE] = createGradientTable();


    // calculate grid points
    let x0 = x.floor() as usize % SIZE;
    let x1 = x0 + 1;
    let y0 = y.floor() as usize % SIZE;
    let y1 = y0 + 1;


    // calculate vectors from each grid corner to point (x,y)
    let vec_bot_left: Vector2D = Vector2D {
        x: x - x0 as f32,
        y: y - y0 as f32
    };
    let vec_bot_right: Vector2D = Vector2D {
        x: x - x1 as f32,
        y: y - y0 as f32
    };
    let vec_top_left: Vector2D = Vector2D {
        x: x - x0 as f32,
        y: y - y1 as f32
    };
    let vec_top_right: Vector2D = Vector2D {
        x: x - x1 as f32,
        y: y - y1 as f32
    };


    // get pseudorandom value (hash) from permutation table for each grid corner
    let val_bot_left: u32 = perm_table[perm_table[x0] as usize + y0];
    let val_bot_right: u32 = perm_table[perm_table[x1] as usize + y0];
    let val_top_left: u32 = perm_table[perm_table[x0] as usize + y1];
    let val_top_right: u32 = perm_table[perm_table[x1] as usize + y1];

    // get associated gradient for each hash
    let grad_bot_left: Vector2D = grad_table[val_bot_left as usize];
    let grad_bot_right: Vector2D = grad_table[val_bot_right as usize];
    let grad_top_left: Vector2D = grad_table[val_top_left as usize];
    let grad_top_right: Vector2D = grad_table[val_top_right as usize];


    // calculate dot product of gradient and vector for each grid corner
    let dot_bot_left = dot(vec_bot_left, grad_bot_left);
    let dot_bot_right = dot(vec_bot_right, grad_bot_right);
    let dot_top_left = dot(vec_top_left, grad_top_left);
    let dot_top_right = dot(vec_top_right, grad_top_right);

    // calculate interpolation weights
    let dx = x - x.floor();
    let dy = y - y.floor();

    // apply fade to weights
    let fade_x = fade(dx);
    let fade_y = fade(dy);

    // perform linear interpolation for left and right sides of grid
    let left_lerp = lerp(fade_y, dot_bot_left, dot_top_left);
    let right_lerp = lerp(fade_y, dot_bot_right, dot_top_right);

    // perform final linear interpolation to get end value
    lerp(fade_x, left_lerp, right_lerp)

}

fn create_permutation_table() -> [u32; SIZE*2] {
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
fn create_gradient_table() -> [Vector2D; SIZE] {
    let mut table: [Vector2D; SIZE] = [Vector2D{x: 0.0, y: 0.0}; SIZE];
    let mut rng = rand::thread_rng();

    for index in 0..SIZE {
        /* rng.gen::<f32>() generates a float between [0,1)
        which is then multiplied by 2 and subtracts 1 to convert the range to [-1.0, 1.0) */
        let mut gradient = Vector2D {
            x: (2.0 * rng.gen::<f32>()) - 1.0, 
            y: (2.0 * rng.gen::<f32>()) - 1.0
        };

        // then normalize the vector
        let gradient_len = (gradient.x.powi(2) + gradient.y.powi(2)).sqrt();
        gradient.x /= gradient_len;
        gradient.y /= gradient_len;


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

// dot product of two vectors
fn dot(a: Vector2D, b: Vector2D) -> f32 {
    (a.x * b.x) + (b.x * b.y)
}

// fade function for smoother interpolation (same as original fade from Ken Perlin)
fn fade(t: f32) -> f32 {
    6.0*t*t*t*t*t - 15.0*t*t*t*t + 10.0*t*t*t
}

// linear interpolation function
fn lerp(weight: f32, dot1: f32, dot2: f32) -> f32 {
    dot1 + weight*(dot2 - dot1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let perm_table = create_permutation_table();
    let grad_table = create_gradient_table();

    // need to generate perm and grid table here to keep pseudorandomness
    //println!("\nnoise val of (0.0, 0.0): {}", noise2D(0.0, 0.0, permTable, gradTable));
    //println!("noise val of (0.1, 0.1): {}", noise2D(0.1, 0.1, permTable, gradTable));

    let mut noise_map = [[0.0; MAPSIZE]; MAPSIZE];
    let mut height_map = [['$'; MAPSIZE]; MAPSIZE];

    // generate 2D noisemap of SIZExSIZE dimensions
    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            let mut noise = noise_2d(x as f32 * 0.01, y as f32 * 0.01, perm_table, grad_table);

            noise += 1.0;
            noise /= 2.0;

            noise_map[y][x] = noise;
        }
        
    }

    let mut img = RgbImage::new(
        MAPSIZE.try_into().unwrap(), MAPSIZE.try_into().unwrap()
    );

    let mut img2 = RgbImage::new(
        MAPSIZE.try_into().unwrap(), MAPSIZE.try_into().unwrap()
    );

    // convert each noise float value to a text tile for viewability
    let mut deep_water_count = 0;
    let mut water_count = 0;
    let mut land_count = 0;
    let mut hill_count = 0;
    let mut mountain_count = 0;

    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            let tile;
            let color;  // color for generating a very basic terrain map
            let color2;         // more accurate color from each float val
            let noise = noise_map[y][x];
            if noise < 0.2 {
                //tile = '*';
                tile = 'W'; // deep water
                color = Rgb([16, 41, 115]); // dark blue

                deep_water_count += 1;
            } else if noise >= 0.2 && noise < 0.4 {
                //tile = '+';
                tile = 'w'; // water
                color = Rgb([45, 83, 196]); // blue
                water_count += 1;
            } else if noise >= 0.4 && noise < 0.6 {
                //tile = '+';
                tile = 'L'; // land
                color = Rgb([18, 135, 31]); // green
                land_count += 1;
            } else if noise >= 0.6 && noise < 0.8 {
                //tile = 'O';
                tile = 'H'; // hill
                color = Rgb([84, 46, 13]); // brown
                hill_count += 1;
            } else {
                //tile = '^';
                tile = 'M'; // mountain
                color = Rgb([65, 65, 65]); // gray
                mountain_count += 1;
            }
            
            let noise_rgb = (noise * 255.0).round() as u8;
            color2 = Rgb([noise_rgb, noise_rgb, noise_rgb]);

            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color);
            img2.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color2);
            height_map[y][x] = tile;
        }
    }

    let file = File::create("heightmap.txt")?;
    let mut file = LineWriter::new(file);
    

    for row in height_map {
        let tile_row = String::from_iter(row);
        file.write_all(tile_row.as_bytes())?;
        file.write_all(b"\n")?;
    }

    file.flush()?;
    img.save("tilemap.png")?;
    img2.save("tilemap2.png")?;

    //println!("noisemap: \n{:#?}", noiseMap);
    /*for row in heightMap  {
        for tile in row {
            print!("{}", tile);
        }
        print!("\n");
        //println!("{:?}", row);
    }*/

    

    println!("Count of deep water tiles: {}", deep_water_count);
    println!("Count of water tiles: {}", water_count);
    println!("Count of land tiles: {}", land_count);
    println!("Count of hill tiles: {}", hill_count);
    println!("Count of mountain tiles: {}", mountain_count);

    Ok(())

}
