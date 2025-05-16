# CSV Tool CLI

A powerful command-line utility for working with CSV files built in Rust. This tool provides several operations for reading, analyzing, searching, and extracting data from CSV files.

## Features

- **Read** CSV files with options to display a limited number of rows and skip headers
- **Get statistics** about CSV files including row/column counts and value distributions
- **Search** for specific terms within columns
- **Extract** specific columns to create new CSV files

## Installation

### Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

### Build from source

1. Clone the repository or download the source code
2. Navigate to the project directory
3. Build the project:

```bash
cargo build --release
```

The executable will be available at `target/release/csv_tool`

## Usage

### Read a CSV file

Display all rows:

```bash
csv_tool read --file data.csv
```

Display only the first 10 rows:

```bash
csv_tool read --file data.csv --head 10
```

Skip the header row:

```bash
csv_tool read --file data.csv --skip-header
```

### Get statistics about a CSV file

```bash
csv_tool stats --file data.csv
```

This provides information about:
- File dimensions (rows and columns)
- Header names
- Empty cell percentage
- Per-column statistics including unique value counts

### Find rows containing a specific term

Search by column name:

```bash
csv_tool find --file data.csv --column "Name" --term "John"
```

Search by column index (0-based):

```bash
csv_tool find --file data.csv --column 2 --term "New York"
```

### Extract specific columns

Extract columns by name:

```bash
csv_tool extract --file input.csv --output extract.csv --columns "Name,Email,Phone"
```

Extract columns by index (0-based):

```bash
csv_tool extract --file input.csv --output extract.csv --columns "0,3,5"
```

You can mix column names and indices:

```bash
csv_tool extract --file input.csv --output extract.csv --columns "Name,3,Email"
```

## Sample Data

For testing purposes, create a sample CSV file:

```csv
Name,Age,City,Email
John Doe,32,New York,john@example.com
Jane Smith,28,Los Angeles,jane@example.com
Bob Johnson,45,Chicago,bob@example.com
Alice Williams,36,Houston,alice@example.com
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.