use crate::api::models::EnvVar;
use crate::error::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct EnvFileReader;

impl EnvFileReader {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<EnvVar>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut env_vars = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                env_vars.push(EnvVar {
                    name: key.trim().to_string(),
                    value: value.trim().to_string(),
                });
            }
        }

        Ok(env_vars)
    }
}
