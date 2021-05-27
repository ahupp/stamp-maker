use image::{GrayImage, imageops, io::Reader as ImageReader};
use std::cmp;
use std::collections::{HashMap};
use image::{Pixel, Luma};
use std::io;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use std::error::Error;

/*
TODO
Expand canvas to handle borders
Alpha
*/

#[derive(Clone, Copy, Debug)]
struct Vert {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct VertIdx {
    idx: u32,
}

impl VertIdx {
    fn new(idx: u32) -> VertIdx {
        VertIdx {idx}
    }
}

struct Face {
    vert: Vec<VertIdx>,
}

impl Face {
    fn tri(a: VertIdx, b: VertIdx, c: VertIdx) -> Face {
        Face {vert: vec![a, b, c]}
    }
}

struct Mesh {
    verts: Vec<Vert>,
    faces: HashMap<u32, Face>,
    index: MeshIndex,
}

struct MeshIndex {
    vert_to_face: HashMap<VertIdx, Vec<u32>>,
    face_to_vert: HashMap<u32, Vec<VertIdx>>,
}

impl Mesh {
    fn new() -> Mesh {
        Mesh {verts: Vec::new(), faces: HashMap::new(),
            index: MeshIndex {face_to_vert: HashMap::new(), vert_to_face: HashMap::new()}}
    }

    fn get_vert<'a>(&'a self, v: VertIdx) -> &'a Vert {
        &self.verts[v.idx as usize]
    }

    fn add_vert(&mut self, x: f64, y: f64, z: f64) -> VertIdx {
        self.verts.push(Vert {x, y, z});
        return VertIdx {idx: (self.verts.len() - 1) as u32};
    }

    fn add_face(&mut self, face: Face) -> u32 {
        let idx : u32 = self.faces.len() as u32;
        self.index.face_to_vert.insert(idx, face.vert.clone());
        for v in &face.vert {
            self.index.vert_to_face.entry(*v).or_insert_with(|| Vec::new()).push(idx);
        }
        self.faces.insert(idx, face);
        return idx
    }

    fn add_quad(&mut self, a: VertIdx, b: VertIdx, c: VertIdx, d: VertIdx) {
        let av = self.get_vert(a);
        let bv = self.get_vert(b);
        let cv = self.get_vert(c);
        let dv = self.get_vert(d);

        fn dist(a: &Vert, b: &Vert) -> f64 {
            fn minsq(c: f64, d: f64) -> f64 {
                (c-d)*(c-d)
            }
            minsq(a.x, b.x) + minsq(a.y, b.y) + minsq(a.z, b.z)
        }

        if dist(&av,&cv) < dist(&bv,&dv) {
            self.add_face(Face::tri(a, b, c));
            self.add_face(Face::tri(a, c, d));
        } else {
            self.add_face(Face::tri(b, c, d));
            self.add_face(Face::tri(a, b, d));
        }
    }

    fn generate_obj(&self, output: &mut dyn io::Write) -> Result<(), std::io::Error> {

        writeln!(output, "g stamp")?;

        for v in &self.verts {
            writeln!(output, "v {} {} {}", v.x, v.y, v.z)?;
        }

        for (_, f) in &self.faces {
            writeln!(output, "f {} {} {}", f.vert[0].idx + 1, f.vert[1].idx + 1, f.vert[2].idx + 1)?;
        }

        return Ok(());
    }
}

fn read_image(file: &str) -> Result<GrayImage, Box<dyn Error>> {
    let img = ImageReader::open(file)?.decode()?;
    Ok(img.into_luma8())
}

fn smooth(img: &GrayImage) -> GrayImage {

    let (xd, yd) = img.dimensions();

    let maybe_get = |x: i32, y: i32|  {
        if x < 0 || y < 0 {
            return None
        }
        let x = x as u32;
        let y = y as u32;
        if x < xd && y < yd {
            Some(img.get_pixel(x, y).channels()[0])
        } else {
            None
        }
    };

    let mut smooth_img = img.clone();
    for y in 0..yd {
        for x in 0..xd {
            let value = img.get_pixel(x, y).channels()[0];

            let next = if value < 255 {
                let mut sc: u32 = 0;
                let mut ss: u32 = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        if let Some(v) = maybe_get(x as i32 + i, y as i32 + j) {
                            sc += 1;
                            ss += v as u32;
                        }
                    }
                }
                let avg = ss / sc;
                ((avg + value as u32)/2) as u8
            } else {
                value
            };
            smooth_img.put_pixel(x, y, Luma([next]));
        }
    }
    return smooth_img;
}

#[wasm_bindgen]
pub struct Options {
    pub invert: bool,
    pub smooth: u32,
    pub max_edge: f64,
    pub height: f64,
}

#[wasm_bindgen]
impl Options {
    // can't export Default trait to wasm
    pub fn new() -> Options {
        Options::default()
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {invert: true, smooth: 10, max_edge: 40.0, height: 3.0}
    }
}

pub fn generate_from_file(file: &str, output: &mut dyn io::Write, opt: &Options) -> Result<(), std::io::Error> {
    let img = read_image(&file)?;
    return generate_raw(img, output, opt)
}
pub fn generate_raw(img: GrayImage, output: &mut dyn io::Write, opt: &Options) -> Result<(), std::io::Error> {

    let mut img = img;
    let (xd, yd) = img.dimensions();

    let maxdim = cmp::max(xd, yd) as f64;
    let scale : f64 = opt.max_edge / maxdim;

    if opt.invert {
      imageops::invert(&mut img);
    }

    for p in img.pixels_mut() {
        p.channels_mut()[0] = if p.channels()[0] > 127 {
            255
        } else {
            0
        }
    }

    for _ in 0..opt.smooth {
        img = smooth(&img);
    }

    let pix_to_z = |x: u32, y: u32| {
        let value = img.get_pixel(x, y).channels()[0] as f64;
        opt.height * (value / 255.0) + 4.0
    };

    let mut mesh = Mesh::new();

    for y in 0..yd {
        for x in 0..xd {
            mesh.add_vert(x as f64 * scale, y as f64 * scale,  pix_to_z(x, y));
        }
    }

    let idx_of = |x: u32, y: u32| VertIdx::new(y*xd + x as u32);

    for y in 0..yd-1 {
        for x in 0..xd-1 {
            let a = idx_of(x, y);
            let b = idx_of(x + 1, y);
            let c = idx_of(x + 1, y + 1);
            let d = idx_of(x, y + 1);
            mesh.add_quad(a, b, c, d);
        }
    }

    // add sides
    let ul = mesh.add_vert(0.0, 0.0, 0.0);
    let ur = mesh.add_vert(xd as f64 * scale, 0.0, 0.0);
    let ll = mesh.add_vert(0.0, yd as f64 * scale, 0.0);
    let lr = mesh.add_vert(xd as f64 * scale, yd as f64 * scale, 0.0);
    mesh.add_quad(idx_of(0, 0), idx_of(0, yd - 1), ll, ul);
    mesh.add_quad(idx_of(xd - 1, 0), idx_of(0, 0), ul, ur);
    mesh.add_quad(idx_of(xd - 1, yd - 1), idx_of(xd - 1, 0), ur, lr);
    mesh.add_quad(idx_of(0, yd - 1), idx_of(xd - 1, yd - 1), lr, ll);

    mesh.generate_obj(output)?;

    return Ok(());
}

pub fn generate_from_bytes(image: &[u8], opt: &Options) -> Result<String, Box<dyn Error>> {
    let reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .expect("Cursor io never fails");
    let img = reader.decode()?;
    let luma = img.into_luma8();
    let mut writer : Vec<u8> = Vec::new();
    generate_raw(luma, &mut writer, opt)?;
    return Ok(String::from_utf8(writer).unwrap());
}

