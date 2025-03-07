use crate::error::Result;
use std::fs::File;
use std::io::{BufRead, BufReader}; // Entfernen von self
use std::path::Path;

pub trait ConfigReader {
    type Output;
    fn read<P: AsRef<Path>>(path: P) -> Result<Self::Output>;
}

pub fn read_file_lines<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = Result<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map(|line| Ok(line?)))
}
