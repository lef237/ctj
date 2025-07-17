# ctj

A command-line tool to convert CSV files to JSON format, supporting both file input and piped stdin.

[![Crates.io](https://img.shields.io/crates/v/ctj.svg)](https://crates.io/crates/ctj)
[![GitHub](https://img.shields.io/badge/github-lef237/ctj-blue.svg)](https://github.com/lef237/ctj)

## Features

- Convert CSV files to JSON with automatic type detection
- Support for piped stdin input (e.g., `cat file.csv | ctj`)
- Support for numbers, booleans, and strings
- Pretty print JSON output
- Output to file or stdout
- Command-line interface with helpful options

## Installation

### Option 1: Install from crates.io (recommended)

```bash
cargo install ctj
```

This will install the latest version from [crates.io](https://crates.io/crates/ctj). After installation, you can use `ctj` command directly from anywhere.

### Option 2: Install from source

Clone the repository and install locally:

```bash
git clone https://github.com/lef237/ctj.git
cd ctj
cargo install --path .
```

### Option 3: Build from source

```bash
git clone https://github.com/lef237/ctj.git
cd ctj
cargo build --release
```

This creates an executable at `./target/release/ctj`.

## Usage

### Basic Usage

Convert CSV to JSON and output to stdout:

```bash
ctj input.csv
```

Or using the explicit flag:

```bash
ctj -i input.csv
```

### Piped Input

You can also pipe CSV data directly to ctj:

```bash
cat input.csv | ctj
echo "name,age\nJohn,30" | ctj
curl https://example.com/data.csv | ctj
```

### Pretty Print

Format JSON output with indentation:

```bash
ctj input.csv -p
```

### Output to File

Save JSON output to a file:

```bash
ctj input.csv -o output.json
```

### Command Line Options

- `-i, --input <FILE>`: Input CSV file (optional, can also be provided as positional argument; if not provided, reads from stdin)
- `-o, --output <FILE>`: Output JSON file (optional, defaults to stdout)
- `-p, --pretty`: Pretty print JSON output
- `-n, --no-header`: Treat the first row as data, not headers (generates column_0, column_1, etc.)
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
ctj sample.csv
```

Output:

```json
[{"name":"John","age":25,"city":"Tokyo","active":true},{"name":"Alice","age":30,"city":"Osaka","active":false},{"name":"Bob","age":35,"city":"Kyoto","active":true}]
```

### Example 2: Pretty printed output

```bash
ctj sample.csv -p
```

Output:

```json
[
  {
    "name": "John",
    "age": 25,
    "city": "Tokyo",
    "active": true
  },
  {
    "name": "Alice",
    "age": 30,
    "city": "Osaka",
    "active": false
  },
  {
    "name": "Bob",
    "age": 35,
    "city": "Kyoto",
    "active": true
  }
]
```

### Example 3: CSV without headers

For CSV files without header rows, use the `-n` option to generate default column names:

```bash
ctj sample-no-header.csv -n -p
```

Given a CSV file `sample-no-header.csv` without headers:

```csv
,,
,,FALSE
,55.5,
```

Output:

```json
[
  {
    "column_0": "",
    "column_1": "",
    "column_2": ""
  },
  {
    "column_0": "",
    "column_1": "",
    "column_2": false
  },
  {
    "column_0": "",
    "column_1": 55.5,
    "column_2": ""
  }
]
```

### Example 4: Using piped input

```bash
cat sample.csv | ctj -p
```

Same output as Example 2.

### Example 5: Processing data from a URL

```bash
curl -s https://example.com/data.csv | ctj --pretty
```

### Example 6: Combining with other commands

```bash
# Process data and save to file
echo "name,score\nAlice,95.5\nBob,87" | ctj > results.json

# Process without headers and pretty print
echo "John,30,Tokyo\nJane,25,Osaka" | ctj --no-header --pretty
```

Output for the last command:

```json
[
  {
    "column_0": "John",
    "column_1": 30,
    "column_2": "Tokyo"
  },
  {
    "column_0": "Jane",
    "column_1": 25,
    "column_2": "Osaka"
  }
]
```

## Type Detection

The tool automatically detects and converts data types:

- **Integers**: Whole numbers (e.g., `25`, `100`) are detected as integers
- **Floating-point numbers**: Numbers with decimal points (e.g., `95.5`, `87.2`) are detected as floats
- **Booleans**: `true`, `false`, `TRUE`, `FALSE`, `True`, `False` are converted to JSON booleans (case-insensitive)
- **Strings**: All other values are treated as strings

## License

This project is available under the MIT License.
