use clap::{Arg, Command};
use csv::Reader;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    input: Option<String>,
    output: Option<String>,
    pretty: bool,
    no_header: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("ctj")
        .about("Convert CSV to JSON from files or piped input")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input CSV file (reads from stdin if not provided)"),
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input CSV file (reads from stdin if not provided)")
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
        .arg(
            Arg::new("no_header")
                .short('n')
                .long("no-header")
                .help("Treat the first row as data, not headers")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_file = matches
        .get_one::<String>("input")
        .or_else(|| matches.get_one::<String>("file"));

    // If no input file specified, we'll read from stdin
    // The error will be handled in convert_csv_to_json if stdin is empty/closed

    let config = Config {
        input: input_file.cloned(),
        output: matches.get_one::<String>("output").cloned(),
        pretty: matches.get_flag("pretty"),
        no_header: matches.get_flag("no_header"),
    };

    convert_csv_to_json(&config)?;

    Ok(())
}

fn parse_boolean(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_number(s: &str) -> Value {
    if let Ok(int_val) = s.parse::<i64>() {
        serde_json::Value::Number(serde_json::Number::from(int_val))
    } else if let Ok(float_val) = s.parse::<f64>() {
        serde_json::Value::Number(serde_json::Number::from_f64(float_val).unwrap())
    } else {
        serde_json::Value::String(s.to_string())
    }
}

fn convert_csv_to_json(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<Box<dyn Read>> = match &config.input {
        Some(file_path) => {
            let file = File::open(file_path)?;
            let boxed_reader: Box<dyn Read> = Box::new(BufReader::new(file));
            if config.no_header {
                csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(boxed_reader)
            } else {
                Reader::from_reader(boxed_reader)
            }
        }
        None => {
            let stdin = io::stdin();
            let boxed_reader: Box<dyn Read> = Box::new(stdin.lock());
            if config.no_header {
                csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(boxed_reader)
            } else {
                Reader::from_reader(boxed_reader)
            }
        }
    };

    let headers = if config.no_header {
        // Generate column names: column_0, column_1, column_2, ...
        let mut all_records = Vec::new();
        let mut max_columns = 0;

        // First pass: collect all records and find max columns
        for result in reader.records() {
            let record = result?;
            max_columns = max_columns.max(record.len());
            all_records.push(record);
        }

        if all_records.is_empty() {
            // Empty file
            let records: Vec<IndexMap<String, Value>> = Vec::new();
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

            return Ok(());
        }

        // Generate headers
        let mut generated_headers = Vec::new();
        for i in 0..max_columns {
            generated_headers.push(format!("column_{}", i));
        }

        // Process all records
        let mut json_records = Vec::new();
        for record in all_records {
            let mut map = IndexMap::new();
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = generated_headers.get(i) {
                    let value: Value = if let Some(bool_val) = parse_boolean(field) {
                        serde_json::Value::Bool(bool_val)
                    } else {
                        parse_number(field)
                    };
                    map.insert(header.to_string(), value);
                }
            }
            json_records.push(map);
        }

        let json_output = if config.pretty {
            serde_json::to_string_pretty(&json_records)?
        } else {
            serde_json::to_string(&json_records)?
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

        return Ok(());
    } else {
        reader.headers()?.clone()
    };

    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut map = IndexMap::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                let value: Value = if let Some(bool_val) = parse_boolean(field) {
                    serde_json::Value::Bool(bool_val)
                } else {
                    parse_number(field)
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
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
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
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: true,
            no_header: false,
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
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
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
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
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
            input: Some("non_existent_file.csv".to_string()),
            output: None,
            pretty: false,
            no_header: false,
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
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
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
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
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
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn test_convert_csv_no_header() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "John,30,Tokyo\nJane,25,Osaka";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: true,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0]["column_0"], "John");
        assert_eq!(parsed[0]["column_1"], 30.0);
        assert_eq!(parsed[0]["column_2"], "Tokyo");
        assert_eq!(parsed[1]["column_0"], "Jane");
        assert_eq!(parsed[1]["column_1"], 25.0);
        assert_eq!(parsed[1]["column_2"], "Osaka");
    }

    #[test]
    fn test_convert_csv_no_header_empty_file() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: true,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn test_integer_vs_float_detection() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "name,age,score,active\nJohn,25,95.5,TRUE\nJane,30,100,False";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 2);

        // First record
        assert_eq!(parsed[0]["name"], "John");
        assert_eq!(parsed[0]["age"], 25); // Integer
        assert_eq!(parsed[0]["score"], 95.5); // Float
        assert_eq!(parsed[0]["active"], true); // Boolean

        // Second record
        assert_eq!(parsed[1]["name"], "Jane");
        assert_eq!(parsed[1]["age"], 30); // Integer
        assert_eq!(parsed[1]["score"], 100); // Integer (not 100.0)
        assert_eq!(parsed[1]["active"], false); // Boolean
    }

    #[test]
    fn test_case_insensitive_booleans() {
        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        let csv_content = "test,value\ncase1,true\ncase2,FALSE\ncase3,True\ncase4,false";
        fs::write(temp_input.path(), csv_content).unwrap();

        let config = Config {
            input: Some(temp_input.path().to_string_lossy().to_string()),
            output: Some(temp_output.path().to_string_lossy().to_string()),
            pretty: false,
            no_header: false,
        };

        convert_csv_to_json(&config).unwrap();

        let output_content = fs::read_to_string(temp_output.path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();

        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[0]["value"], true);
        assert_eq!(parsed[1]["value"], false);
        assert_eq!(parsed[2]["value"], true);
        assert_eq!(parsed[3]["value"], false);
    }
}
