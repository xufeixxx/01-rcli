pub mod base64;
pub mod csv;
pub mod genpass;

use std::path::Path;

use self::{base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, and convert CSV to other format")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Encode and decode base64")]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

// 对verify_input_file做unit test, 最好在同一个文件中
#[cfg(test)]
mod tests {
    use crate::cli::verify_input_file;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Result::Ok("-".into()));
        assert_eq!(
            verify_input_file("*"),
            Result::Err("File does not exist".into())
        );
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Result::Ok("Cargo.toml".into())
        );
    }
}
