use std::fs::File;
use std::io::prelude::*;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() -> () {
    let path = "image.ppm";
    let mut fs = File::create(path).expect("File creation failed");
    write!(fs, "P6\n{} {}\n255\n", WIDTH, HEIGHT).expect("Header failed");

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let ir: u8 = (255.999 * (i as f32 / (WIDTH-1) as f32)) as u8;
            let ig: u8 = (255.999 * (j as f32 / (HEIGHT-1) as f32)) as u8;
            let ib: u8 = (255.999 * 0.25) as u8;
            
            let mut buffer: Vec<u8> = Vec::new();
            buffer.push(ir);
            buffer.push(ig);
            buffer.push(ib);

            fs.write_all(&buffer).expect("write failed");
        }
    }
    println!("Hello, world!");
}
