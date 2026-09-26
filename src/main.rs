use std::fs::File;
use std::io::{self,Write};
use std::io::BufWriter;

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

            let ir = (255.999 * r) as usize;
            let ig = (255.999 * g) as usize;
            let ib = (255.999 * b) as usize;

            writeln!(writer,"{} {} {}", ir, ig, ib)?;
        }
    }

    eprint!("\rDone!");
    Ok(())

}
