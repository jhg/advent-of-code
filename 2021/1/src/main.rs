use structopt::StructOpt;

use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(StructOpt)]
struct Opt {
    input_file: PathBuf,
}

impl Opt {
    fn lines() -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
        let opt = Self::from_args();
        let file = io::BufReader::new(File::open(opt.input_file)?);
        return Ok(file.lines());
    }
}

fn main() -> io::Result<()> {
    let mut prev_depth_measure = None;
    let mut total_times_increase = 0;
    for line in Opt::lines()? {
        let depth_measure: usize = line?.parse().expect("Can not parse the depth measurement");
        if let Some(prev_depth_measure) = prev_depth_measure {
            if prev_depth_measure < depth_measure {
                println!("{} (increased)", depth_measure);
                total_times_increase += 1;
            } else if prev_depth_measure > depth_measure {
                println!("{} (decreased)", depth_measure);
            } else {
                println!("{} (keep)", depth_measure);
            }
        } else {
            println!("{} (N/A - no previous measurement)", depth_measure);
        }
        prev_depth_measure = Some(depth_measure);
    }
    println!("{}", total_times_increase);
    return Ok(());
}
