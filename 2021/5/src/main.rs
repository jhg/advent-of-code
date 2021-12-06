use structopt::StructOpt;

use scanf::sscanf;

use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};

mod diagram;
use diagram::HydrothermalDiagram;

#[derive(StructOpt)]
struct Opt {
    input_file: PathBuf,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let input_file = io::BufReader::new(File::open(opt.input_file)?);
    let mut diagram = HydrothermalDiagram::default();
    for line in input_file.lines() {
        let line = line?;
        let (mut x1, mut y1, mut x2, mut y2) = (0usize, 0usize, 0usize, 0usize);
        sscanf!(&line, "{},{} -> {},{}", x1, y1, x2, y2)?;
        diagram.line(x1, y1, x2, y2);
    }
    if diagram.x_len() < 100 {
        println!("{}", diagram);
    }
    println!("{}", diagram.total_danger_points());
    return Ok(());
}
