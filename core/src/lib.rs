use image::{GrayImage, imageops, io::Reader as ImageReader};
use std::cmp;
use std::collections::{HashMap};
use image::{ImageBuffer, Luma, Pixel};
use std::io;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use std::error::Error;


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

        // Split the quad into triangles so that we minimize the length of the hypotenuse
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

// Convert into a single channel image, copying from alpha
fn flatten_alpha<T: 'static + Pixel<Subpixel = u8>>(img: &ImageBuffer<T, Vec<T::Subpixel>>) -> GrayImage {
    let (xd, yd) = img.dimensions();
    let mut dst: ImageBuffer<Luma<T::Subpixel>, Vec<T::Subpixel>> = ImageBuffer::new(xd, yd);
    let alpha_channel : usize = (T::CHANNEL_COUNT - 1) as usize;
    for (x, y, pixel) in dst.enumerate_pixels_mut() {
        let src_pix = img.get_pixel(x, y);
        let channels = src_pix.channels();
        let alpha = channels[alpha_channel];
        pixel[0] = alpha;
    }
    dst
}


fn read_image(file: &str) -> Result<GrayImage, Box<dyn Error>> {
    let img = ImageReader::open(file)?.decode()?;
    let gray = match img {
        image::DynamicImage::ImageLumaA8(img) => flatten_alpha(&img),
        image::DynamicImage::ImageRgba8(img) => flatten_alpha(&img),
        image::DynamicImage::ImageLuma8(img) => img,
        _ => panic!("unhandled image type: {:?}", img.color()),
    };
    Ok(gray)
}

fn smooth(img: &GrayImage, smooth_radius_pixels: u32) -> GrayImage {

    let (xd, yd) = img.dimensions();

    // List of coordinates within a circle of smooth_radius
    let mut circle = Vec::new();
    for dx in 0..(smooth_radius_pixels + 1) {
        for dy in 0..(smooth_radius_pixels + 1) {
            if dx*dx + dy*dy <= smooth_radius_pixels*smooth_radius_pixels {
                circle.push((dx as i32, dy as i32));
                if !(dx == 0 && dy == 0) {
                    circle.push((-(dx as i32), dy as i32));
                    circle.push((dx as i32, -(dy as i32)));
                    circle.push((-(dx as i32), -(dy as i32)));
                }
            }
        }
    }

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

    let smooth_radius = smooth_radius_pixels as f32;

    let mut smooth_img = img.clone();
    for (x, y, pixel) in smooth_img.enumerate_pixels_mut() {

        // Squared distance to nearest set pixel
        let nearest_peak_squared = circle.iter().filter_map(|(dx, dy)| {
            let (x, y) = (dx + x as i32, dy + y as i32);
            if let Some(value) = maybe_get(x, y) {
                if value == 255 {
                    return Some(dx*dx + dy*dy)
                }
            }
            return None
        }).min();

        if let Some(nearest_peak_squared) = nearest_peak_squared {
            let nearest_peak = (nearest_peak_squared as f32).sqrt();

            let value = 255.0 * (smooth_radius - nearest_peak) / smooth_radius;

            pixel[0] = value as u8;
        }
    }
    return smooth_img;
}

#[wasm_bindgen]
pub struct Options {
    pub invert: bool,
    pub smooth_radius_mm: f64,
    pub max_edge_mm: f64,
    pub height_mm: f64,
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
        Options {invert: true, smooth_radius_mm: 0.5, max_edge_mm: 40.0, height_mm: 3.0}
    }
}

pub fn generate_from_file(file: &str, output: &mut dyn io::Write, opt: &Options) -> Result<(), Box<dyn Error>> {
    let img = read_image(&file)?;
    Ok(generate_raw(img, output, opt)?)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn generate_raw(img: GrayImage, output: &mut dyn io::Write, opt: &Options) -> Result<(), std::io::Error> {

    let mm_per_pixel = |img: &GrayImage| {
        let (xd, yd) = img.dimensions();
        let maxdim = cmp::max(xd, yd);
        opt.max_edge_mm / (maxdim as f64)
    };

    let mut img = img;

    // Convert to a binary image, 255 is high, 0 is low
    if opt.invert {
        imageops::invert(&mut img);
    }

    // Resize if image is large, since the conversion is slow
    img = {
        let (xd, yd) = img.dimensions();
        let maxdim = cmp::max(xd, yd);
        const MAX_DIMENSION : u32 = 512;
        if maxdim > MAX_DIMENSION {
            let scale = |dim: u32| {
                let rescale = (MAX_DIMENSION as f32) / (maxdim as f32);
                (rescale * (dim as f32)).round() as u32
            };
            let (rxd, ryd) = (scale(xd), scale(yd));
            imageops::resize(&img, rxd, ryd, imageops::FilterType::Nearest)
        } else {
            img
        }
    };

    for p in img.pixels_mut() {
        p.channels_mut()[0] = if p.channels()[0] > 127 {
            255
        } else {
            0
        }
    }

    // push out border to avoid holes on edge
    img = {
        let border_padding = (opt.smooth_radius_mm / mm_per_pixel(&img) + 5.0) as u32;
        let mut expanded = GrayImage::new(
            img.dimensions().0 + 2*border_padding, img.dimensions().1 + 2*border_padding);
        imageops::replace(&mut expanded, &img, border_padding, border_padding);
        expanded
    };

    if opt.smooth_radius_mm > 0.0 {
        let smooth_radius_pixels = (opt.smooth_radius_mm / mm_per_pixel(&img)) as u32;
        img = smooth(&img, smooth_radius_pixels);
    }

    let pix_to_z = |pix: &Luma<u8>| {
        let value = pix[0] as f64;
        opt.height_mm * (value / 255.0) + 4.0
    };

    let mut mesh = Mesh::new();

    let mm_per_pixel = mm_per_pixel(&img);
    for (x, y, pixel) in img.enumerate_pixels() {
        mesh.add_vert(x as f64 * mm_per_pixel, y as f64 * mm_per_pixel,  pix_to_z(pixel));
    }

    let (xd, yd) = img.dimensions();
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
    let ur = mesh.add_vert(xd as f64 * mm_per_pixel, 0.0, 0.0);
    let ll = mesh.add_vert(0.0, yd as f64 * mm_per_pixel, 0.0);
    let lr = mesh.add_vert(xd as f64 * mm_per_pixel, yd as f64 * mm_per_pixel, 0.0);
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

