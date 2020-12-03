use image::{io::Reader as ImageReader};
use std::env;
use std::cmp;
use std::collections::HashMap;
use image::{Pixel, Luma};

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

fn main()  -> Result<(), std::io::Error> {

    let file = env::args().nth(1).unwrap();
    // TODO: not sure how to handle decode error
    let img = ImageReader::open(file)?.decode().unwrap();

    let imgn = img.into_luma8();

    let (xd, yd) = imgn.dimensions();

    let max_edge = 50.0;

    let maxdim = cmp::max(xd, yd) as f64;
    let scale : f64 = max_edge / maxdim;

    let mut vert_to_idx = HashMap::new();

    // OBJ file verticies start at 1
    let mut vert_ctr= 1;

    let mut vert_of = |x: u32, y: u32, z: f64| {
        let v = Vert::new(x as f64 * scale, y as f64 * scale, z);
        let idx = vert_to_idx.entry(v).or_insert_with(||{
            vert_ctr += 1;
            vert_ctr
        });
        return *idx;
    };


    let mut faces = Vec::new();

    let mut add_face = |a: u32, b: u32, c: u32, d: u32| {
        faces.push((a, b, d));
        faces.push((b, c, d));
    };

    for y in 0..yd-1 {
        for x in 0..xd-1 {
            let px = imgn.get_pixel(x, y);

            let pix_to_z = |px: &Luma<u8>| {
                // TODO: select height
                let value = px.channels()[0];
                return 1.0;
                if value == 0 {
                    4.0
                } else {
                    1.0
                }
            };

            let z = pix_to_z(px);

            add_face(
                vert_of(x, y, z), vert_of(x, y+1, z),
                vert_of(x+1, y+1, z), vert_of(x, y+1, z)
            );

            // check face below

            let pxb = imgn.get_pixel(x, y + 1);

            if px != pxb && false {
                let zb = pix_to_z(pxb);
                let (top, bot) = if z > zb {
                    (z, zb)
                } else {
                    (zb, z)
                };

                add_face(
                    vert_of(x, y + 1, z), vert_of(x + 1, y + 1, z),
                    vert_of(x + 1, y + 1, zb), vert_of(x, y + 1, z)
                );
            }

            let pxn = imgn.get_pixel(x + 1, y);
            if px != pxn && false {
                let zn = pix_to_z(pxn);
                add_face(
                    vert_of(x + 1, y, z), vert_of(x + 1, y, zn),
                    vert_of(x + 1, y + 1, zn), vert_of(x + 1, y + 1, z)
                );
            }
        }
    }

    let mut verts : Vec<(Vert, u32)> = vert_to_idx.into_iter().collect();
    verts.sort_by_key(|(_, idx)| *idx);

    println!("g Object001");

    for (vert, _) in &verts {
        let (x, y, z) = vert.to_float();
        println!("v {} {} {}", x, y, z);
    }

    for (a, b, c) in faces {
        println!("f {} {} {}", a, b, c);
    }

    return Ok(());
}
