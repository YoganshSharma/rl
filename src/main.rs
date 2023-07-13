use std::{path::PathBuf, io::{Read, stdin}};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    files: Option<Vec<String>>,
    #[arg(short='v', long="starting-number")]
    start: Option<isize>,
}

fn main() {
    let cli = Cli::parse();
    let mut buf = String::new();
    for f in cli.files.unwrap_or(vec!["-".to_owned()]) {
        match f.as_str() {
            "-" => {
                stdin().read_to_string(&mut buf).unwrap();
            },
            _ => {
                buf.push_str(&std::fs::read_to_string(PathBuf::from(f)).unwrap());
             },
        }
    }

    let mut i= cli.start.unwrap_or(1);
    for val in buf.lines() {
        if val.is_empty() {
            println!();
        }
        else {
            println!("{}\t{val}",i);
            i = i.checked_add(1).expect("integer overflow");
        }
    }
}

