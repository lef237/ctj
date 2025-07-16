use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_cli_malformed_csv() {
    let temp_input = NamedTempFile::new().unwrap();

    let csv_content = "name,age\nJohn,30\nJane,25,extra_field";
    fs::write(temp_input.path(), csv_content).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-i"])
        .arg(temp_input.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_cli_empty_csv_file() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();

    fs::write(temp_input.path(), "").unwrap();

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

    assert_eq!(parsed.len(), 0);
}

#[test]
fn test_cli_invalid_output_path() {
    let temp_input = NamedTempFile::new().unwrap();

    let csv_content = "name,age\nJohn,30";
    fs::write(temp_input.path(), csv_content).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-i"])
        .arg(temp_input.path())
        .arg("-o")
        .arg("/invalid/path/output.json")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_cli_permission_denied() {
    let temp_input = NamedTempFile::new().unwrap();

    let csv_content = "name,age\nJohn,30";
    fs::write(temp_input.path(), csv_content).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-i"])
        .arg(temp_input.path())
        .arg("-o")
        .arg("/etc/output.json")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_cli_unicode_csv() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();

    let csv_content = "名前,年齢,都市\n田中,30,東京\n佐藤,25,大阪";
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
    assert_eq!(parsed[0]["名前"], "田中");
    assert_eq!(parsed[0]["年齢"], 30);
    assert_eq!(parsed[0]["都市"], "東京");
}

#[test]
fn test_cli_special_characters_in_csv() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();

    let csv_content = "name,description\nJohn,\"Hello, World!\"\nJane,\"Line 1\nLine 2\"";
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
    assert_eq!(parsed[0]["description"], "Hello, World!");
    assert_eq!(parsed[1]["name"], "Jane");
    assert_eq!(parsed[1]["description"], "Line 1\nLine 2");
}

#[test]
fn test_cli_large_numbers() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();

    let csv_content = "name,big_number,small_number\nTest,999999999999999,0.0000000001";
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

    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["name"], "Test");
    assert_eq!(parsed[0]["big_number"], 999999999999999_f64);
    assert_eq!(parsed[0]["small_number"], 0.0000000001);
}

#[test]
fn test_cli_mixed_data_types() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();

    let csv_content = "name,age,active,score,notes\nJohn,30,true,95.5,Good student\nJane,25,false,88.0,\nBob,,true,92.3,Excellent";
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

    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0]["name"], "John");
    assert_eq!(parsed[0]["age"], 30);
    assert_eq!(parsed[0]["active"], true);
    assert_eq!(parsed[0]["score"], 95.5);
    assert_eq!(parsed[0]["notes"], "Good student");

    assert_eq!(parsed[1]["name"], "Jane");
    assert_eq!(parsed[1]["age"], 25);
    assert_eq!(parsed[1]["active"], false);
    assert_eq!(parsed[1]["score"], 88.0);
    assert_eq!(parsed[1]["notes"], "");

    assert_eq!(parsed[2]["name"], "Bob");
    assert_eq!(parsed[2]["age"], "");
    assert_eq!(parsed[2]["active"], true);
    assert_eq!(parsed[2]["score"], 92.3);
    assert_eq!(parsed[2]["notes"], "Excellent");
}

#[test]
fn test_cli_integer_vs_float_detection() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();

    let csv_content = "name,age,score,active\nJohn,25,95.5,TRUE\nJane,30,100,False";
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

    // First record - verify integer vs float detection
    assert_eq!(parsed[0]["name"], "John");
    assert_eq!(parsed[0]["age"], 25); // Should be integer
    assert_eq!(parsed[0]["score"], 95.5); // Should be float
    assert_eq!(parsed[0]["active"], true); // Should be boolean (case-insensitive)

    // Second record
    assert_eq!(parsed[1]["name"], "Jane");
    assert_eq!(parsed[1]["age"], 30); // Should be integer
    assert_eq!(parsed[1]["score"], 100); // Should be integer (not 100.0)
    assert_eq!(parsed[1]["active"], false); // Should be boolean (case-insensitive)
}

#[test]
fn test_cli_case_insensitive_booleans() {
    let temp_input = NamedTempFile::new().unwrap();
    let temp_output = NamedTempFile::new().unwrap();

    let csv_content = "test,value\ncase1,true\ncase2,FALSE\ncase3,True\ncase4,false";
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

    assert_eq!(parsed.len(), 4);
    assert_eq!(parsed[0]["value"], true);
    assert_eq!(parsed[1]["value"], false);
    assert_eq!(parsed[2]["value"], true);
    assert_eq!(parsed[3]["value"], false);
}
