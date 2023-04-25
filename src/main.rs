use std::{env, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut out_string: String = String::new();
    if args.is_empty() {
        println!("No arguments found!");
        process::exit(1);
    }
    for file in args {
        let mut value: String =
            fs::read_to_string(Path::new(file.as_str())).expect("Cannot read file");
        value.insert_str(0, "\n");
        out_string.push_str(value.as_str());
    }
    println!("{}", out_string.trim_start());
}
