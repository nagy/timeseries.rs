use gnuplot::{Color, Figure, RGBString};
use std::{env, error::Error, fs::File};

use timeseries::io::csv;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).unwrap();
    let file = File::open(file_path)?;
    let ts = csv::read(file)?;

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&ts.index.values, &ts.values, &[Color(RGBString("blue"))]);
    fg.show()?;
    Ok(())
}
