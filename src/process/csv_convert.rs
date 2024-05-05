use anyhow::Ok;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::cli::csv::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut player_vec = Vec::with_capacity(128);
    let header = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // header.iter() -> 使用header的迭代器
        // record.iter() -> 使用record的迭代器
        // zip()将两个迭代器合并成一个元组迭代器[(header, record), ...]
        // collect::<Value()> -> 将元组迭代器转换成JSON Value
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        // println!("{:?}", json_value);
        player_vec.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&player_vec)?,
        // OutputFormat::Toml => toml::to_string_pretty(&player_vec)?,
        OutputFormat::Yaml => serde_yaml::to_string(&player_vec)?,
    };
    fs::write(output, content)?;
    Ok(())
}
