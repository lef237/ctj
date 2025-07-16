# ctj

A command-line tool to convert CSV files to JSON format.

## Features

- Convert CSV files to JSON with automatic type detection
- Support for numbers, booleans, and strings
- Pretty print JSON output
- Output to file or stdout
- Command-line interface with helpful options

## Installation

### Option 1: Install globally (recommended)

```bash
cargo install --path .
```

After installation, you can use `ctj` command directly from anywhere.

### Option 2: Build from source

```bash
cargo build --release
```

This creates an executable at `./target/release/ctj`.

## Usage

### Basic Usage

Convert CSV to JSON and output to stdout:

```bash
ctj -i input.csv
```

### Pretty Print

Format JSON output with indentation:

```bash
ctj -i input.csv -p
```

### Output to File

Save JSON output to a file:

```bash
ctj -i input.csv -o output.json
```

### Command Line Options

- `-i, --input <FILE>`: Input CSV file (required)
- `-o, --output <FILE>`: Output JSON file (optional, defaults to stdout)
- `-p, --pretty`: Pretty print JSON output
- `-h, --help`: Show help message
- `-V, --version`: Show version information

## Examples

Given a CSV file `sample.csv`:

```csv
name,age,city,active
John,25,Tokyo,true
Alice,30,Osaka,false
Bob,35,Kyoto,true
```

### Example 1: Basic conversion

```bash
ctj -i sample.csv
```

Output:
```json
[{"name":"John","active":true,"city":"Tokyo","age":25.0},{"active":false,"name":"Alice","city":"Osaka","age":30.0},{"city":"Kyoto","age":35.0,"name":"Bob","active":true}]
```

### Example 2: Pretty printed output

```bash
ctj -i sample.csv -p
```

Output:
```json
[
  {
    "city": "Tokyo",
    "age": 25.0,
    "active": true,
    "name": "John"
  },
  {
    "active": false,
    "age": 30.0,
    "name": "Alice",
    "city": "Osaka"
  },
  {
    "age": 35.0,
    "active": true,
    "name": "Bob",
    "city": "Kyoto"
  }
]
```

## Type Detection

The tool automatically detects and converts data types:

- **Numbers**: Detected as floating-point numbers
- **Booleans**: `true` and `false` are converted to JSON booleans
- **Strings**: All other values are treated as strings

## License

This project is available under the MIT License.
