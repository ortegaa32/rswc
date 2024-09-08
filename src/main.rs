use std::io::{self, BufRead,BufReader};
use std::fs::File;
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

// consts to use as array indexes
const BYTES: usize = 0;
const WORDS: usize = 1;
const LINES: usize = 2;
const MULTI: usize = 3;

fn main() -> io::Result<()>{
    let args = Cli::parse();

    let mut totals: [u64; 4] = [0; 4];

    for path in &args.path {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let mut counts: [u64; 4] = [0; 4];
        
        get_counts(&mut counts, &mut reader, &mut totals);

        print_report(&args, counts, path); 
    }

    if args.path.len() > 1 {
        print_totals(&args, &mut totals);
    }
    Ok(())
}

fn get_counts(counts: &mut [u64; 4], reader: &mut BufReader<File>, 
        totals: &mut [u64; 4]) {
    let mut line_string = String::new();

    while reader.read_line(&mut line_string).unwrap() > 0 {

        // Count number of bytes per line
        let byte_length: u64 = line_string.len() as u64;
        counts[BYTES] += byte_length;

        // Count number of words per line
        let words: Vec<&str> = line_string.split_whitespace().collect();
        let filtered_words: Vec<&str> = words.into_iter()
            .filter(|word| word.len() > 0).collect();
        let word_count: u64 = filtered_words.len() as u64;
        counts[WORDS] += word_count;

        // Increase line count per iteration
        counts[LINES] += 1;

        // Count number of multibyte characters per line
        let multi_count: u64 = line_string.chars().count() as u64;
        counts[MULTI] += multi_count;

        line_string.clear();
    }

    totals[BYTES] += counts[BYTES];
    totals[WORDS] += counts[WORDS];
    totals[LINES] += counts[LINES];
    totals[MULTI] += counts[MULTI];

}

// Function to print results of command
fn print_report(args: &Cli, counts: [u64; 4], file_path: &PathBuf) {
    // Indicates if user entered command with no flags
    let default_option: bool = !(args.bytes || args.lines || args.words 
        || args.multibytes);

    let mut report = String::new();

    if default_option || args.lines {
        report.push_str(&format!("{:>8}", counts[LINES]));
    }
    
    if default_option || args.words {
        report.push_str(&format!("{:>8}", counts[WORDS]));
    }

    if default_option || args.bytes {
        report.push_str(&format!("{:>8}", counts[BYTES]));
    }

    if args.multibytes && !args.bytes {
        report.push_str(&format!("{:>8}", counts[MULTI]));
    }

    report.push_str(&format!(" {}", file_path.display()));

    println!("{}", report);
}

fn print_totals(args: &Cli, totals: &mut [u64; 4]) {
    let default_option: bool = !(args.bytes || args.lines || args.words 
        || args.multibytes);

    let mut report = String::new();

    if default_option || args.lines {
        report.push_str(&format!("{:>8}", totals[LINES]));
    }
    
    if default_option || args.words {
        report.push_str(&format!("{:>8}", totals[WORDS]));
    }

    if default_option || args.bytes {
        report.push_str(&format!("{:>8}", totals[BYTES]));
    }

    if args.multibytes && !args.bytes {
        report.push_str(&format!("{:>8}", totals[MULTI]));
    }

    report.push_str(&" total");

    println!("{}", report);
}
