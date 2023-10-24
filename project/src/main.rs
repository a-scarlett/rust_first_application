use std::env;
use std::fs;

fn main() {
    let args = env::args_os();

    let directory_path = args.skip(1).next();

    if let Some(directory_path) = directory_path {
        for entry in fs::read_dir(directory_path).unwrap() {
            let file_name = entry.unwrap().file_name();
            println!("{}", file_name.to_string_lossy());
        }
    } else {
        eprintln!("Please give directory path");
    }
}
