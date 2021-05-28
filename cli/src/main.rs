
use std::io;
use std::env;
use stamp_maker;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), io::Error> {
    let file = env::args().nth(1).unwrap();
    let opt = stamp_maker::Options::default();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut f = File::open(file)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;

    let s = stamp_maker::generate_from_bytes(&data, &opt).unwrap();
    stdout.write_all(&s.as_bytes())?;

    return Ok(());
}