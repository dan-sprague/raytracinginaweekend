use std::fs::File;
use std::io::{self,Write};
use std::io::BufWriter;

mod vec3;
mod ray;
mod color;
use vec3::{Vec3,Color};
use color::write_color;
fn main() -> io::Result<()> {

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);

    let width: i64 = 256;
    let height: i64 = 256;

    writeln!(writer,"P3\n{} {}\n255", width, height)?;

    for j in 0..height {
        eprint!("\rScanlines remaining: {}",(height - j));
        for i in 0..width {
            let r = i as f64 / width as f64;
            let g = j as f64 / height as f64;
            let b = 0.0;

            let pixel_color = Color(r,g,b);

            write_color(&mut writer,&pixel_color)?;


            
        }
    }

    eprint!("\rDone!");
    Ok(())

}

