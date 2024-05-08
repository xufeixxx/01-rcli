pub mod base64;
pub mod csv;
pub mod genpass;
pub mod http;
pub mod text;

use self::{
    base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts, http::HttpSubCommand,
    text::TextSubCommand,
};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

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
    #[command(subcommand, name = "text", about = "")]
    Text(TextSubCommand),
    #[command(subcommand, name = "http")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

// 对verify_input_file做unit test, 最好在同一个文件中
#[cfg(test)]
mod tests {
    use crate::cli::verify_file;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Result::Ok("-".into()));
        assert_eq!(verify_file("*"), Result::Err("File does not exist".into()));
        assert_eq!(verify_file("Cargo.toml"), Result::Ok("Cargo.toml".into()));
    }
}
