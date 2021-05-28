
use std::io;
use std::env;
use stamp_maker;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), io::Error> {
    let file = env::args().nth(1).unwrap();
    let mut opt = stamp_maker::Options::default();
    opt.max_edge = 40.0;
    opt.invert = true;
    opt.smooth = 100;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut f = File::open(file)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data);

    let s = stamp_maker::generate_from_bytes(&data, &opt).unwrap();

    return Ok(());
}