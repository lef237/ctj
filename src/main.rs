use clap::{Arg, Command};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

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
                .help("Input CSV file"),
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input CSV file")
                .index(1),
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

    let input_file = matches
        .get_one::<String>("input")
        .or_else(|| matches.get_one::<String>("file"))
        .ok_or("Input file is required")?;

    let config = Config {
        input: input_file.clone(),
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
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(field.parse().unwrap()).unwrap(),
                    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_convert_csv_to_json_basic() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name,age,city\nJohn,30,Tokyo\nJane,25,Osaka";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: temp_input.path().to_string_lossy().to_string(),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0]["name"], "John");
        assert_eq!(parsed[0]["age"], 30.0);
        assert_eq!(parsed[0]["city"], "Tokyo");
        assert_eq!(parsed[1]["name"], "Jane");
        assert_eq!(parsed[1]["age"], 25.0);
        assert_eq!(parsed[1]["city"], "Osaka");
    }

    #[test]
    fn test_convert_csv_to_json_pretty() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name,active\nTest,true";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: temp_input.path().to_string_lossy().to_string(),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: true,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        assert!(output_content.contains("  "));
        assert!(output_content.contains("\n"));

        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();
        assert_eq!(parsed[0]["name"], "Test");
        assert_eq!(parsed[0]["active"], true);
    }

    #[test]
    fn test_convert_csv_with_numbers_and_booleans() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name,score,passed\nAlice,95.5,true\nBob,80,false";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: temp_input.path().to_string_lossy().to_string(),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed[0]["name"], "Alice");
        assert_eq!(parsed[0]["score"], 95.5);
        assert_eq!(parsed[0]["passed"], true);
        assert_eq!(parsed[1]["name"], "Bob");
        assert_eq!(parsed[1]["score"], 80.0);
        assert_eq!(parsed[1]["passed"], false);
    }

    #[test]
    fn test_convert_csv_empty_fields() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name,age,city\nJohn,,\n,25,Osaka";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: temp_input.path().to_string_lossy().to_string(),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed[0]["name"], "John");
        assert_eq!(parsed[0]["age"], "");
        assert_eq!(parsed[0]["city"], "");
        assert_eq!(parsed[1]["name"], "");
        assert_eq!(parsed[1]["age"], 25.0);
        assert_eq!(parsed[1]["city"], "Osaka");
    }

    #[test]
    fn test_convert_csv_file_not_found() {
        let config = Config {
            input: "non_existent_file.csv".to_string(),
            output: None,
            pretty: false,
        };

        let result = convert_csv_to_json(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_csv_invalid_format() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name,age\nJohn,30\nJane";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: temp_input.path().to_string_lossy().to_string(),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
        };

        let result = convert_csv_to_json(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_csv_single_column() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name\nJohn\nJane";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: temp_input.path().to_string_lossy().to_string(),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0]["name"], "John");
        assert_eq!(parsed[1]["name"], "Jane");
    }

    #[test]
    fn test_convert_csv_no_data_rows() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name,age,city";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: temp_input.path().to_string_lossy().to_string(),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 0);
    }
}
