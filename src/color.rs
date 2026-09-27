use std::io::BufWriter;
use std::fs::File;
use std::io::Result;
use std::io::{Write};

use crate::vec3::{Color};


pub fn write_color(buf: &mut BufWriter<File>, pixel_color: &Color) -> Result<()>  {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rybte = (255.999 * r) as usize;
    let gbyte = (255.999 * g) as usize;
    let bbyte = (255.999 * b) as usize;

    writeln!(buf,"{} {} {}", rybte, gbyte, bbyte)?;


    Ok(())

}