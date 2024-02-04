use std::io::{self, Write};

fn main() {
    let xs = vec![1, 2, 3];
    println!("The list is: {:?}", xs);
    println!{"this is information"};
    eprintln!("this is an error message");

    let stdout = io::stdout();
    // let mut handle = io::BufWriter::new(stdout);
    let mut handle = stdout.lock();
    writeln! (handle, "foo: {:?}", 42);
}