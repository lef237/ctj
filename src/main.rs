use clap::{Arg, Command};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    input: String,
    output: Option<String>,
    pretty: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("ctj")
        .about("Convert CSV to JSON")
        .version("0.1.0")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input CSV file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output JSON file (default: stdout)"),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Pretty print JSON output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config = Config {
        input: matches.get_one::<String>("input").unwrap().clone(),
        output: matches.get_one::<String>("output").cloned(),
        pretty: matches.get_flag("pretty"),
    };

    convert_csv_to_json(&config)?;

    Ok(())
}

fn convert_csv_to_json(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.input)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    
    let headers = reader.headers()?.clone();
    let mut records = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        let mut map = HashMap::new();
        
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                let value: Value = if field.parse::<f64>().is_ok() {
                    serde_json::Value::Number(serde_json::Number::from_f64(field.parse().unwrap()).unwrap())
                } else if field.parse::<bool>().is_ok() {
                    serde_json::Value::Bool(field.parse().unwrap())
                } else {
                    serde_json::Value::String(field.to_string())
                };
                map.insert(header.to_string(), value);
            }
        }
        
        records.push(map);
    }
    
    let json_output = if config.pretty {
        serde_json::to_string_pretty(&records)?
    } else {
        serde_json::to_string(&records)?
    };
    
    match &config.output {
        Some(output_file) => {
            std::fs::write(output_file, json_output)?;
            println!("JSON output written to: {}", output_file);
        }
        None => {
            println!("{}", json_output);
        }
    }
    
    Ok(())
}
