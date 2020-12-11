use image::{ImageBuffer, io::Reader as ImageReader};
use std::env;
use std::cmp;
use std::collections::HashMap;
use image::{Pixel, Luma, imageops, GenericImageView};

#[derive(Eq, PartialEq, Hash)]
struct Vert {
    v: (u64, u64, u64)
}

impl Vert {
    fn new(x: f64, y: f64, z: f64) -> Vert {
        Vert {v: (x.to_bits(), y.to_bits(), z.to_bits())}
    }

    fn to_float(&self) -> (f64, f64, f64) {
        (f64::from_bits(self.v.0), f64::from_bits(self.v.1), f64::from_bits(self.v.2))
    }
}

fn read_image(file: &str) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, std::io::Error> {
    // TODO: not sure how to handle decode error
    let img = ImageReader::open(file)?.decode().unwrap();

    let luma = img.into_luma8();
    return Ok(luma);
}


fn main()  -> Result<(), std::io::Error> {

    let file = env::args().nth(1).unwrap();

    let img = read_image(&file)?;

    let (oxd, oyd) = img.dimensions();
    let img = imageops::resize(&img, oxd*2, oyd*2, image::imageops::FilterType::Lanczos3);
    let (xd, yd) = img.dimensions();

    let max_edge = 20.0;

    let maxdim = cmp::max(xd, yd) as f64;
    let scale : f64 = max_edge / maxdim;


    let base_height = 1.0;

    let pix_to_z = |x: u32, y: u32| {
        let value = img.get_pixel(x, y).channels()[0];
        if value != 0 {
            base_height
        } else {
            2.0
        }
    };

    let mut vert_to_idx = HashMap::new();

    // OBJ file verticies start at 1
    let mut vert_ctr= 1;

    let mut vert_of = |x: u32, y: u32| {
        let z = pix_to_z(x, y);
        let v = Vert::new(x as f64 * scale, y as f64 * scale, z);
        let idx = vert_to_idx.entry(v).or_insert_with(||{
            let cur = vert_ctr;
            vert_ctr += 1;
            cur
        });
        return *idx;
    };

    let mut faces = Vec::new();

    let mut add_face = |a: u32, b: u32, c: u32, d: u32| {
        faces.push((a, b, c));
        faces.push((c, b, d));
    };
    for y in 0..yd-1 {
        for x in 0..xd-1 {
            add_face(
                vert_of(x, y), vert_of(x + 1, y),
                vert_of(x, y+1), vert_of(x + 1, y + 1),
            )
        }
    }



    let mut verts : Vec<(Vert, u32)> = vert_to_idx.into_iter().collect();
    verts.sort_by_key(|(_, idx)| *idx);

    println!("g stamp");

    for (vert, _) in &verts {
        let (x, y, z) = vert.to_float();
        println!("v {} {} {}", x, y, z);
    }

    for (a, b, c) in faces {
        println!("f {} {} {}", a, b, c);
    }

    return Ok(());
}
