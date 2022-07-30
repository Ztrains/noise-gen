use std::time::Instant;
use std::{error::Error};

use image::{RgbImage, Rgb};

use noise_gen::noise_2d;
use noise_gen::table::*;


const MAPSIZE: usize = 500;
const OCTAVES: u32 = 8;


fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let perm_table = create_permutation_table();
    let grad_table = create_gradient_table();

    let mut noise_map = [[0.0; MAPSIZE]; MAPSIZE];

    // generate 2D heightmap of MAPSIZExMAPSIZE dimensions
    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            let mut noise = 0.0;
            let mut freq = 0.005;
            let mut ampl = 1.0;

            /* when OCTAVES > 1, Fractal Brownian Motion is used, which layers
            multiple octaves of noise for a more realistic fractal look on edges */
            for _ in 0..OCTAVES {
                let val = ampl * noise_2d(x as f32 * freq, y as f32 * freq, perm_table, grad_table);
                noise += val;

                ampl *= 0.5;
                freq *= 2.0;

            }

            // adding octaves increase the range of noise output, need to get it back between (-1.0, 1.0)
            //noise /= 2.0 - 2.0_f32.powi(1 - OCTAVES as i32);
            //noise *= SQRT_2 / 2.0;

            noise += 1.0;
            noise *= 0.5;

            // can optimize by adding to img here instead of adding to noise_map
            noise_map[y][x] = noise;
        }
        
    }

    let mut img = RgbImage::new(
        MAPSIZE.try_into().unwrap(), MAPSIZE.try_into().unwrap()
    );

    // get counts of each heightmap terrain tile for debugging
    let mut avg: f32 = 0.0;
    let mut lowest = 1.0;
    let mut highest = 0.0;


    /* instead of iterating through noise_map here for image generation, 
    can be optimized by doing this inside the noise loop above */
    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            let color;  // color of tile for heightmap
            let noise = noise_map[y][x];
            if noise < 0.2 {
                color = Rgb([16, 41, 115]);     // dark blue (deep water)
            } else if noise < 0.4 {
                color = Rgb([45, 83, 196]);     // blue (water)
            } else if noise < 0.45 {
                color = Rgb([235, 204, 150]);   // tan (beach)
            } else if noise < 0.65 {
                color = Rgb([18, 135, 31]);     // green (land)
            } else if noise < 0.8 {
                color = Rgb([84, 46, 13]);      // brown (hill)
            } else if noise < 0.95 {
                color = Rgb([65, 65, 65]);      // gray (mountain)
            } else {
                color = Rgb([250, 250, 250]);   //white (snow)
            }
            
            // apply grayscale instead of color
            /*let noise_gray = (noise * 255.0).round() as u8;
            color = Rgb([noise_gray, noise_gray, noise_gray]);*/

            
            img.put_pixel(x as u32, y as u32, color);
            
            // calculating avg, high, and low noise values
            avg += noise;
            if noise < lowest {
                lowest = noise;
            } else if noise > highest{
                highest = noise;
            }
        }
    }

    img.save("tilemap.png")?;


    avg /= (MAPSIZE * MAPSIZE) as f32;
    println!("average noise value: {}", avg);
    println!("lowest noise value: {}", lowest);
    println!("highest noise value: {}", highest);

    println!("Time elapsed: {:?}", now.elapsed());
    Ok(())

}
