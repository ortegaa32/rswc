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
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    println!("Hello, world!");
    println!("Args: {:?}", args);
}
