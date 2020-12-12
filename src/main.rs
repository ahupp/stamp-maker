use image::{ImageBuffer, io::Reader as ImageReader};
use std::env;
use std::cmp;
use std::collections::{HashMap, HashSet};
use image::{Pixel, Luma, imageops};


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

    fn print_obj(&self) {
        println!("g stamp");

        for v in &self.verts {
            println!("v {} {} {}", v.x, v.y, v.z)
        }

        for (_, f) in &self.faces {
            println!("f {} {} {}", f.vert[0].idx + 1, f.vert[1].idx + 1, f.vert[2].idx + 1);
        }
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

    let (xd, yd) = img.dimensions();

    let max_edge = 20.0;

    let maxdim = cmp::max(xd, yd) as f64;
    let scale : f64 = max_edge / maxdim;

    let pix_to_z = |x: u32, y: u32| {
        let value = img.get_pixel(x, y).channels()[0];
        if value != 0 {
            2.0
        } else {
            1.0
        }
    };

    let mut mesh = Mesh::new();

    for y in 0..yd {
        for x in 0..xd {
            mesh.add_vert(x as f64 * scale, y as f64 * scale,  pix_to_z(x, y));
            //mesh.add_vert(x as f64 * scale, y as f64 * scale,  (3.14159*(x as f64)/50.).sin());
        }
    }

    let idx_of = |x: u32, y: u32| (y*xd + x) as u32;

    for y in 0..yd-1 {
        for x in 0..xd-1 {
            let a = VertIdx::new(idx_of(x, y));
            let b = VertIdx::new(idx_of(x + 1, y));
            let c = VertIdx::new(idx_of(x, y + 1));
            let d = VertIdx::new(idx_of(x + 1, y + 1));
            mesh.add_face(Face::tri(a, b, c));
            mesh.add_face(Face::tri(c, b, d));
        }
    }

    mesh.print_obj();

    return Ok(());
}
