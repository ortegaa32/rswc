use std::io::{self, BufRead,BufReader};
use std::fs::File;
use std::collections::HashMap;
use clap::Parser;
//use unicode_segmentation::UnicodeSegmentation;

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
    path: std::path::PathBuf
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    let mut counts: HashMap<&str, u64> = HashMap::new();
    counts.insert("bytes", 0);
    counts.insert("words", 0);
    counts.insert("lines", 0);
    counts.insert("multibytes", 0);
    
    get_counts(&mut counts, &mut reader);

    print_report(args, counts);

    Ok(())
}

fn get_counts(counts: &mut HashMap<&str, u64>, reader: &mut BufReader<File>) {
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
}

// Function to print results of command
fn print_report(args: Cli, counts: HashMap<&str, u64>) {
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

    report.push_str(&format!(" {}", args.path.display()));

    println!("{}", report);
}
