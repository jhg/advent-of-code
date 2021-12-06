use structopt::StructOpt;

use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(StructOpt)]
pub struct Opt {
    input_file: PathBuf,
}

impl Opt {
    pub fn from_args() -> Self { StructOpt::from_args() } // This allow to use the struct without import the trait.

    /// Will read lines from the input file while there is not I/O errors.
    pub fn lines(&self) -> io::Result<impl Iterator<Item=String>> {
        let lines = io::BufReader::new(File::open(&self.input_file)?)
            .lines()
            .take_while(|line| line.is_ok())
            .map(|line| line.unwrap());
        return Ok(lines);
    }
}
