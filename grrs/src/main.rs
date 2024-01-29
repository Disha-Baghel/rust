use clap::Parser;
use std::io::BufReader; 

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::parse();

    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}