
use std::io;
use std::env;

use img2obj;

fn main() -> Result<(), io::Error> {
    let file = env::args().nth(1).unwrap();
    let mut opt = img2obj::Options::default();
    opt.max_edge = 40.0;
    opt.invert = true;
    opt.smooth = 100;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    img2obj::generate(&file, &mut stdout, opt)?;

    return Ok(());
}