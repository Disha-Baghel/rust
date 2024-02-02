use clap::Parser;
use std::io::BufReader; 
use std::io::BufRead;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()>{
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::parse();

    // let file = std::fs::File::open(&args.path)?;
    // let reader = BufReader::new(file);

    // for line in reader.lines() {
    //     let line = line?;
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    let result = std::fs::read_to_string(&args.path)?;
    let content = match result {
        Ok(content) => {content},
        Err(e) => {panic!("can't read file: {}", e);}
    };
    println!("{}", content);
    
    Ok(())

}
    