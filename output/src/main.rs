// use std::io::{self, Write};

fn main() {
//     let xs = vec![1, 2, 3];
//     println!("The list is: {:?}", xs);
//     println!{"this is information"};
//     eprintln!("this is an error message");

//     let stdout = io::stdout();
//     // let mut handle = io::BufWriter::new(stdout);
//     let mut handle = stdout.lock();
//     writeln! (handle, "foo: {:?}", 42);

// Showing a progress bar
        let pb = indicatif::ProgressBar::new(100);
        for i in 0..100 {
            do_hard_work();
            pb.println(format!("[+] finished #{}", i+1));
            pb.inc(1);
        }
        pb.finish_with_message("done");
}

fn do_hard_work() {
    println!("doing hard work");
}