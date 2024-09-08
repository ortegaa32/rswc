use std::io::{self, BufRead,BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short='c')]
    bytes: bool,
    #[arg(short='w')]
    words: bool,
    #[arg(short='l')]
    lines: bool,
    #[arg(short='m')]
    multibytes: bool,
    path: Vec<std::path::PathBuf>
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    let mut totals: HashMap<&str, u64> = HashMap::new();
        totals.insert("bytes", 0);
        totals.insert("words", 0);
        totals.insert("lines", 0);
        totals.insert("multibytes", 0);

    for path in &args.path {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let mut counts: HashMap<&str, u64> = HashMap::new();
        counts.insert("bytes", 0);
        counts.insert("words", 0);
        counts.insert("lines", 0);
        counts.insert("multibytes", 0);
        
        get_counts(&mut counts, &mut reader, &mut totals);

        print_report(&args, counts, path); 
    }

    if args.path.len() > 1 {
        print_totals(&args, &mut totals);
    }
    Ok(())
}

fn get_counts(counts: &mut HashMap<&str, u64>, reader: &mut BufReader<File>, 
    totals: &mut HashMap<&str, u64>) {
    let mut line_string = String::new();

    while reader.read_line(&mut line_string).unwrap() > 0 {

        // Count number of bytes per line
        let byte_length: u64 = line_string.len() as u64;
        counts.entry("bytes").and_modify(|k| *k += byte_length);

        // Count number of words per line
        let words: Vec<&str> = line_string.split_whitespace().collect();
        let filtered_words: Vec<&str> = words.into_iter()
            .filter(|word| word.len() > 0).collect();
        let word_count: u64 = filtered_words.len() as u64;
        counts.entry("words").and_modify(|k| *k += word_count);

        // Increase line count per iteration
        counts.entry("lines").and_modify(|k| *k += 1);

        // Count number of multibyte characters per line
        let multi_count: u64 = line_string.chars().count() as u64;
        counts.entry("multibytes").and_modify(|k| *k += multi_count);

        line_string.clear();
    }

    totals.entry("bytes").and_modify(|k| *k += counts["bytes"]);
    totals.entry("words").and_modify(|k| *k += counts["words"]);
    totals.entry("lines").and_modify(|k| *k += counts["lines"]);
    totals.entry("multibytes").and_modify(|k| *k += counts["multibytes"]);

}

// Function to print results of command
fn print_report(args: &Cli, counts: HashMap<&str, u64>, file_path: &PathBuf) {
    // Indicates if user entered command with no flags
    let default_option: bool = !(args.bytes || args.lines || args.words 
        || args.multibytes);

    let mut report = String::new();

    if default_option || args.lines {
        report.push_str(&format!("{:>8}", counts["lines"]));
    }
    
    if default_option || args.words {
        report.push_str(&format!("{:>8}", counts["words"]));
    }

    if default_option || args.bytes {
        report.push_str(&format!("{:>8}", counts["bytes"]));
    }

    if args.multibytes && !args.bytes {
        report.push_str(&format!("{:>8}", counts["multibytes"]));
    }

    report.push_str(&format!(" {}", file_path.display()));

    println!("{}", report);
}

fn print_totals(args: &Cli, totals: &mut HashMap<&str, u64>) {
    let default_option: bool = !(args.bytes || args.lines || args.words 
        || args.multibytes);

    let mut report = String::new();

    if default_option || args.lines {
        report.push_str(&format!("{:>8}", totals["lines"]));
    }
    
    if default_option || args.words {
        report.push_str(&format!("{:>8}", totals["words"]));
    }

    if default_option || args.bytes {
        report.push_str(&format!("{:>8}", totals["bytes"]));
    }

    if args.multibytes && !args.bytes {
        report.push_str(&format!("{:>8}", totals["multibytes"]));
    }

    report.push_str(&" total");

    println!("{}", report);
}
