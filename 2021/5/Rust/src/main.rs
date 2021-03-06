use helpers::Opt;

use scanf::sscanf;

use std::io;

mod diagram;
use diagram::HydrothermalDiagram;

fn main() -> io::Result<()> {
    let mut diagram = HydrothermalDiagram::default();
    for line in Opt::from_args().lines()? {
        let (mut x1, mut y1, mut x2, mut y2) = (0usize, 0usize, 0usize, 0usize);
        sscanf!(&line, "{},{} -> {},{}", x1, y1, x2, y2)?;
        diagram.add_line(x1, y1, x2, y2);
    }
    if diagram.x_len() < 100 {
        println!("{}", diagram);
    }
    println!("{}", diagram.total_danger_points());
    return Ok(());
}
