use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_cli_basic_conversion() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();
    
    let csv_content = "name,age,city\nJohn,30,Tokyo\nJane,25,Osaka";
    fs::write(temp_input.path(), csv_content).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-i"])
        .arg(temp_input.path())
        .arg("-o")
        .arg(temp_output.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let output_content = fs::read_to_string(temp_output.path()).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();
    
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0]["name"], "John");
    assert_eq!(parsed[0]["age"], 30.0);
    assert_eq!(parsed[0]["city"], "Tokyo");
}

#[test]
fn test_cli_pretty_output() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();
    
    let csv_content = "name,active\nTest,true";
    fs::write(temp_input.path(), csv_content).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-i"])
        .arg(temp_input.path())
        .arg("-o")
        .arg(temp_output.path())
        .arg("--pretty")
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let output_content = fs::read_to_string(temp_output.path()).unwrap();
    assert!(output_content.contains("  "));
    assert!(output_content.contains("\n"));
    
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();
    assert_eq!(parsed[0]["name"], "Test");
    assert_eq!(parsed[0]["active"], true);
}

#[test]
fn test_cli_positional_argument() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();
    
    let csv_content = "name,score\nAlice,95.5\nBob,80";
    fs::write(temp_input.path(), csv_content).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .arg(temp_input.path())
        .arg("-o")
        .arg(temp_output.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let output_content = fs::read_to_string(temp_output.path()).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();
    
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0]["name"], "Alice");
    assert_eq!(parsed[0]["score"], 95.5);
    assert_eq!(parsed[1]["name"], "Bob");
    assert_eq!(parsed[1]["score"], 80.0);
}

#[test]
fn test_cli_stdout_output() {
    let temp_input = NamedTempFile::new().unwrap();
    
    let csv_content = "name,age\nJohn,30";
    fs::write(temp_input.path(), csv_content).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-i"])
        .arg(temp_input.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["name"], "John");
    assert_eq!(parsed[0]["age"], 30.0);
}

#[test]
fn test_cli_nonexistent_file() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-i", "nonexistent.csv"])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
}

#[test]
fn test_cli_no_input_file() {
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Convert CSV to JSON"));
    assert!(stdout.contains("--input"));
    assert!(stdout.contains("--output"));
    assert!(stdout.contains("--pretty"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("ctj 0.1.0"));
}