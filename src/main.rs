use std::{fs::File, io::{LineWriter, Write}};

use rand::Rng;


const SIZE: usize = 256;
//const permTable: [u32; SIZE*2] = createPermutationTable();
//const gradTable: [Vector2D; SIZE] = createGradientTable();

#[derive(Clone, Copy, Debug)]
struct Vector2D {
    x: f32,
    y: f32
}

fn noise2D(x: f32, y: f32, permTable: [u32; SIZE*2], gradTable: [Vector2D; SIZE]) -> f32 {

    //let permTable: [u32; SIZE*2] = createPermutationTable();
    //let gradTable: [Vector2D; SIZE] = createGradientTable();


    // calculate grid points
    let x0 = x.floor() as usize % SIZE;
    let x1 = x0 + 1;
    let y0 = y.floor() as usize % SIZE;
    let y1 = y0 + 1;


    // calculate vectors from each grid corner to point (x,y)
    let vecBotLeft: Vector2D = Vector2D {
        x: x - x0 as f32,
        y: y - y0 as f32
    };
    let vecBotRight: Vector2D = Vector2D {
        x: x - x1 as f32,
        y: y - y0 as f32
    };
    let vecTopLeft: Vector2D = Vector2D {
        x: x - x0 as f32,
        y: y - y1 as f32
    };
    let vecTopRight: Vector2D = Vector2D {
        x: x - x1 as f32,
        y: y - y1 as f32
    };


    // get pseudorandom value (hash) from permutation table for each grid corner
    let valBotLeft: u32 = permTable[permTable[x0] as usize + y0];
    let valBotRight: u32 = permTable[permTable[x1] as usize + y0];
    let valTopLeft: u32 = permTable[permTable[x0] as usize + y1];
    let valTopRight: u32 = permTable[permTable[x1] as usize + y1];

    // get associated gradient for each hash
    let gradBotLeft: Vector2D = gradTable[valBotLeft as usize];
    let gradBotRight: Vector2D = gradTable[valBotRight as usize];
    let gradTopLeft: Vector2D = gradTable[valTopLeft as usize];
    let gradTopRight: Vector2D = gradTable[valTopRight as usize];


    // calculate dot product of gradient and vector for each grid corner
    let dotBotLeft = dot(vecBotLeft, gradBotLeft);
    let dotBotRight = dot(vecBotRight, gradBotRight);
    let dotTopLeft = dot(vecTopLeft, gradTopLeft);
    let dotTopRight = dot(vecTopRight, gradTopRight);

    // calculate interpolation weights
    let dx = x - x.floor();
    let dy = y - y.floor();

    // apply fade to weights
    let fadeX = fade(dx);
    let fadeY = fade(dy);

    // perform linear interpolation for left and right sides of grid
    let leftLerp = lerp(fadeY, dotBotLeft, dotTopLeft);
    let rightLerp = lerp(fadeY, dotBotRight, dotTopRight);

    // perform final linear interpolation to get end value
    lerp(fadeX, leftLerp, rightLerp)

}

fn createPermutationTable() -> [u32; SIZE*2] {
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

fn createGradientTable() -> [Vector2D; SIZE] {
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
        let gradientLen = (gradient.x.powi(2) + gradient.y.powi(2)).sqrt();
        gradient.x /= gradientLen;
        gradient.y /= gradientLen;


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
    let mut randIndex;

    while i > 0 {
        randIndex = rng.gen_range(0..i+1);

        //println!("before swap for i={}:", i);
        //println!("arr[i]={}, arr[randIndex]={}", arr[i], arr[randIndex]);

        // swap arr[i] with arr[randIndex]
        arr.swap(i, randIndex);

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

fn main() -> std::io::Result<()> {
    let permTable = createPermutationTable();
    let gradTable = createGradientTable();

    // need to generate perm and grid table here to keep pseudorandomness
    //println!("\nnoise val of (0.0, 0.0): {}", noise2D(0.0, 0.0, permTable, gradTable));
    //println!("noise val of (0.1, 0.1): {}", noise2D(0.1, 0.1, permTable, gradTable));

    let mut noiseMap = [[0.0; SIZE]; SIZE];
    let mut heightMap = [['$'; SIZE]; SIZE];

    // generate 2D noisemap of SIZExSIZE dimensions
    for y in 0..SIZE {
        for x in 0..SIZE {
            let mut noise = noise2D(x as f32 * 0.01, y as f32 * 0.01, permTable, gradTable);

            noise += 1.0;
            noise /= 2.0;

            noiseMap[y][x] = noise;
        }
        
    }

    // convert each noise float value to a text tile for viewability
    for y in 0..SIZE {
        for x in 0..SIZE {
            let tile;
            let noise = noiseMap[y][x];
            if noise < 0.4 {
                //tile = '*';
                tile = 'W'; // water
            } else if noise >= 0.3 && noise < 0.6 {
                //tile = '+';
                tile = 'L'; // land
            } else if noise >= 0.6 && noise < 0.9 {
                //tile = 'O';
                tile = 'H'; // hill
            } else {
                //tile = '^';
                tile = 'M'; // mountain
            }

            heightMap[y][x] = tile;
        }
    }

    let file = File::create("heightmap.txt")?;
    let mut file = LineWriter::new(file);

    for row in heightMap {
        let tileRow = String::from_iter(row);
        file.write_all(tileRow.as_bytes())?;
        file.write_all(b"\n")?;
    }

    file.flush()?;

    //println!("noisemap: \n{:#?}", noiseMap);
    /*for row in heightMap  {
        for tile in row {
            print!("{}", tile);
        }
        print!("\n");
        //println!("{:?}", row);
    }*/

    Ok(())

}
