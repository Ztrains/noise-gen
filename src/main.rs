
use std::{error::Error};

use image::{RgbImage, Rgb};

use noise_gen::noise_2d;
use noise_gen::table::*;


const MAPSIZE: usize = 500;
const OCTAVES: u32 = 8;


fn main() -> Result<(), Box<dyn Error>> {
    
    let perm_table = create_permutation_table();
    let mut noise_map = [[0.0; MAPSIZE]; MAPSIZE];

    // generate 2D heightmap of MAPSIZExMAPSIZE dimensions
    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            let mut noise = 0.0;    
            let mut freq = 0.005;   // smaller frequency creates a more "zoomed-in" look
            let mut ampl = 1.0;     // smaller amplitude means less height variance between peaks and valleys

            /* when OCTAVES > 1, Fractal Brownian Motion is used, which layers
            multiple octaves of noise for a more realistic fractal output on edges */
            for _ in 0..OCTAVES {
                let val = ampl * noise_2d(x as f32 * freq, y as f32 * freq, perm_table);
                noise += val;

                ampl *= 0.55;    // smaller values create a smoother output vertically (actual noise value)
                freq *= 2.0;    // smaller values create a smoother output horizontally (difference between adjacent values)
            }

            // change range of noise values from ±1 to 0-1
            noise += 1.0;
            noise *= 0.5;

            // can optimize by adding to img here instead of adding to noise_map
            noise_map[y][x] = noise;
        }
    }

    let mut img = RgbImage::new(
        MAPSIZE.try_into().unwrap(), MAPSIZE.try_into().unwrap()
    );

    /* instead of iterating through noise_map here for image generation, 
    can be optimized by doing this inside the noise loop above */
    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            let color;  // color of tile for heightmap
            let noise = noise_map[y][x];
            if noise < 0.225 {
                color = Rgb([16, 41, 115]);     // dark blue (deep water)
            } else if noise < 0.45 {
                color = Rgb([45, 83, 196]);     // blue (water)
            } else if noise < 0.50 {
                color = Rgb([235, 204, 150]);   // tan (beach)
            } else if noise < 0.65 {
                color = Rgb([18, 135, 31]);     // green (land)
            } else if noise < 0.85 {
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
        }
    }

    img.save("tilemap.png")?;
    Ok(())
}
