use std::{fs::File, io::{LineWriter, Write}, error::Error};

use image::{RgbImage, Rgb};

use noise_gen::noise_2d;
use noise_gen::table::*;


const MAPSIZE: usize = 500;
const OCTAVES: u32 = 8;


fn main() -> Result<(), Box<dyn Error>> {
    let perm_table = create_permutation_table();
    let grad_table = create_gradient_table();

    let mut noise_map = [[0.0; MAPSIZE]; MAPSIZE];
    //let mut height_map = [['$'; MAPSIZE]; MAPSIZE];

    // generate 2D noisemap of SIZExSIZE dimensions
    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            let mut noise = 0.0;
            let mut freq = 0.005;
            let mut ampl = 1.0;

            for _ in 0..OCTAVES {
                let val = ampl * noise_2d(x as f32 * freq, y as f32 * freq, perm_table, grad_table);
                noise += val;

                ampl *= 0.5;
                freq *= 2.0;

            }


            noise += 1.0;
            noise *= 0.5;

            noise_map[y][x] = noise;
        }
        
    }

    let mut img = RgbImage::new(
        MAPSIZE.try_into().unwrap(), MAPSIZE.try_into().unwrap()
    );

    /*let mut img2 = RgbImage::new(
        MAPSIZE.try_into().unwrap(), MAPSIZE.try_into().unwrap()
    );*/

    // convert each noise float value to a text tile for viewability
    let mut deep_water_count = 0;
    let mut water_count = 0;
    let mut land_count = 0;
    let mut hill_count = 0;
    let mut mountain_count = 0;

    let mut avg: f32 = 0.0;
    let mut lowest = 1.0;
    let mut highest = 0.0;

    for y in 0..MAPSIZE {
        for x in 0..MAPSIZE {
            //let tile;
            let color;  // color for generating a very basic terrain map
            //let color2;         // more accurate color from each float val
            let noise = noise_map[y][x];
            if noise < 0.2 {
                //tile = '*';
                //tile = 'W'; // deep water
                color = Rgb([16, 41, 115]); // dark blue
                deep_water_count += 1;
            } else if noise >= 0.2 && noise < 0.4 {
                //tile = '+';
                //tile = 'w'; // water
                color = Rgb([45, 83, 196]); // blue
                water_count += 1;
            } else if noise >= 0.4 && noise < 0.6 {
                //tile = '+';
                //tile = 'L'; // land
                color = Rgb([18, 135, 31]); // green
                land_count += 1;
            } else if noise >= 0.6 && noise < 0.8 {
                //tile = 'O';
                //tile = 'H'; // hill
                color = Rgb([84, 46, 13]); // brown
                hill_count += 1;
            } else {
                //tile = '^';
                //tile = 'M'; // mountain
                color = Rgb([65, 65, 65]); // gray
                mountain_count += 1;
            }
            
            /*let noise_rgb = (noise * 255.0).round() as u8;
            color2 = Rgb([noise_rgb, noise_rgb, noise_rgb]);*/

            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color);
            /*img2.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color2);
            height_map[y][x] = tile;*/
            avg += noise;
            if noise < lowest {
                lowest = noise;
            } else if noise > highest{
                highest = noise;
            }
        }
    }
    

    let file = File::create("heightmap.txt")?;
    let mut file = LineWriter::new(file);
    

    /*for row in height_map {
        let tile_row = String::from_iter(row);
        file.write_all(tile_row.as_bytes())?;
        file.write_all(b"\n")?;
    }*/

    file.flush()?;
    img.save("tilemap.png")?;
    //img2.save("tilemap2.png")?;

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

    avg /= (MAPSIZE * MAPSIZE) as f32;
    println!("average noise value: {}", avg);
    println!("lowest noise value: {}", lowest);
    println!("highest noise value: {}", highest);

    Ok(())

}
