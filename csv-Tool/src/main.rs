use clap::{Parser, Subcommand};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "csv_tool")]
#[command(author = "Rust Developer")]
#[command(version = "1.0")]
#[command(about = "A CSV file processing utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Read {
       
        #[arg(short, long)]
        file: PathBuf,

      
        #[arg(short, long, default_value_t = 0)]
        head: usize,

        
        #[arg(short, long, default_value_t = false)]
        skip_header: bool,
    },
    
    Stats {
        /// CSV file path
        #[arg(short, long)]
        file: PathBuf,
    },
    /// Find rows matching a search term
    Find {
        /// CSV file path
        #[arg(short, long)]
        file: PathBuf,

        /// Column to search in (name or index)
        #[arg(short, long)]
        column: String,

        /// Term to search for
        #[arg(short, long)]
        term: String,
    },
    /// Extract specific columns from CSV
    Extract {
        /// Input CSV file path
        #[arg(short, long)]
        file: PathBuf,

        /// Output CSV file path
        #[arg(short, long)]
        output: PathBuf,

        /// Columns to extract (comma separated names or indices)
        #[arg(short, long)]
        columns: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Read {
            file,
            head,
            skip_header,
        } => {
            read_csv(file, *head, *skip_header)?;
        }
        Commands::Stats { file } => {
            display_stats(file)?;
        }
        Commands::Find { file, column, term } => {
            find_in_csv(file, column, term)?;
        }
        Commands::Extract {
            file,
            output,
            columns,
        } => {
            extract_columns(file, output, columns)?;
        }
    }

    Ok(())
}

fn read_csv(file: &PathBuf, head: usize, skip_header: bool) -> Result<(), Box<dyn Error>> {
    let file = File::open(file)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(file);

    let headers = reader.headers()?.clone();
    let mut records: Vec<StringRecord> = Vec::new();

    // Skip header if requested
    if !skip_header {
        print_record(&headers, 0, true)?;
        println!("{}", "-".repeat(80));
    }

    // Read and store records
    for (i, result) in reader.records().enumerate() {
        let record = result?;
        records.push(record.clone());
        
        // Print immediately if no head limit or within head limit
        if head == 0 || i < head {
            print_record(&record, i + 1, false)?;
        }
    }

    // Summary
    println!("{}", "-".repeat(80));
    println!("Total rows: {}", records.len());

    Ok(())
}

fn print_record(record: &StringRecord, row_num: usize, is_header: bool) -> Result<(), Box<dyn Error>> {
    let row_indicator = if is_header { "H" } else { &row_num.to_string() };
    
    print!("{:>5} | ", row_indicator);
    
    for (i, field) in record.iter().enumerate() {
        if i > 0 {
            print!(" | ");
        }
        
        // Truncate long fields for display
        if field.len() > 20 {
            print!("{}...", &field[0..17]);
        } else {
            print!("{}", field);
        }
    }
    
    println!();
    Ok(())
}

fn display_stats(file: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(file)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(file_content.as_bytes());

    let headers = reader.headers()?.clone();
    
    // Get basic stats
    let mut row_count = 0;
    let mut column_count = headers.len();
    let mut empty_cells = 0;
    let mut column_stats: Vec<HashMap<String, usize>> = vec![HashMap::new(); column_count];
    
    for result in reader.records() {
        let record = result?;
        row_count += 1;
        
        for (i, field) in record.iter().enumerate() {
            if field.is_empty() {
                empty_cells += 1;
            }
            
            // Count unique values for each column
            if let Some(column_map) = column_stats.get_mut(i) {
                *column_map.entry(field.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    // Print the statistics
    println!("📊 CSV File Statistics: {}", file.display());
    println!("---------------------------------------------------");
    println!("📏 Dimensions: {} rows × {} columns", row_count, column_count);
    println!("🔤 Headers: {}", headers.iter().collect::<Vec<_>>().join(", "));
    println!("📉 Empty cells: {} ({:.2}%)", 
        empty_cells, 
        (empty_cells as f64 / (row_count * column_count) as f64) * 100.0
    );
    println!();
    
    // Print column-specific stats
    println!("📋 Column Statistics:");
    for (i, col_name) in headers.iter().enumerate() {
        if let Some(col_stats) = column_stats.get(i) {
            let unique_values = col_stats.len();
            let most_common = col_stats
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(val, count)| (val, *count));
                
            println!("  {} [{}]:", i + 1, col_name);
            println!("    - Unique values: {}", unique_values);
            if let Some((val, count)) = most_common {
                println!("    - Most common: \"{}\" ({} times, {:.1}%)", 
                    val,
                    count,
                    (count as f64 / row_count as f64) * 100.0
                );
            }
        }
    }
    
    Ok(())
}

fn find_in_csv(file: &PathBuf, column: &str, term: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(file);
    
    let headers = reader.headers()?.clone();
    let column_index = if let Ok(idx) = column.parse::<usize>() {
        // If column is a number, use it as index (0-based)
        if idx >= headers.len() {
            return Err(format!("Column index {} out of range (0-{})", 
                idx, headers.len() - 1).into());
        }
        idx
    } else {
        // If column is a name, find its index
        match headers.iter().position(|h| h == column) {
            Some(idx) => idx,
            None => return Err(format!("Column '{}' not found in headers", column).into()),
        }
    };
    
    println!("🔍 Searching for \"{}\" in column \"{}\":", term, headers[column_index]);
    println!("{}", "-".repeat(80));
    
    // Print headers
    print_record(&headers, 0, true)?;
    println!("{}", "-".repeat(80));
    
    let mut matches = 0;
    
    for (row_idx, result) in reader.records().enumerate() {
        let record = result?;
        
        // Check if the term is in the specified column
        if let Some(field) = record.get(column_index) {
            if field.to_lowercase().contains(&term.to_lowercase()) {
                print_record(&record, row_idx + 1, false)?;
                matches += 1;
            }
        }
    }
    
    println!("{}", "-".repeat(80));
    println!("Found {} matching rows", matches);
    
    Ok(())
}

fn extract_columns(input: &PathBuf, output: &PathBuf, columns: &str) -> Result<(), Box<dyn Error>> {
    // Parse column specifications
    let column_specs: Vec<&str> = columns.split(',').collect();
    
    // Open the input file
    let input_file = File::open(input)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(input_file);
    
    let headers = reader.headers()?.clone();
    
    // Resolve column indices
    let mut column_indices = Vec::new();
    for spec in column_specs {
        let spec = spec.trim();
        
        if let Ok(idx) = spec.parse::<usize>() {
            // If spec is a number, use it as index (0-based)
            if idx >= headers.len() {
                return Err(format!("Column index {} out of range (0-{})", 
                    idx, headers.len() - 1).into());
            }
            column_indices.push(idx);
        } else {
            // If spec is a name, find its index
            match headers.iter().position(|h| h == spec) {
                Some(idx) => column_indices.push(idx),
                None => return Err(format!("Column '{}' not found in headers", spec).into()),
            }
        }
    }
    
    // Create output file and writer
    let output_file = File::create(output)?;
    let mut writer = WriterBuilder::new().from_writer(output_file);
    
    // Write header row
    let mut header_record = StringRecord::new();
    for &idx in &column_indices {
        header_record.push_field(&headers[idx]);
    }
    writer.write_record(&header_record)?;
    
    // Write data rows
    let mut count = 0;
    for result in reader.records() {
        let record = result?;
        let mut new_record = StringRecord::new();
        
        for &idx in &column_indices {
            if let Some(field) = record.get(idx) {
                new_record.push_field(field);
            } else {
                new_record.push_field("");
            }
        }
        
        writer.write_record(&new_record)?;
        count += 1;
    }
    
    writer.flush()?;
    
    println!("✅ Successfully extracted {} columns to {}", 
        column_indices.len(), 
        output.display());
    println!("   Processed {} rows", count);
    
    Ok(())
}