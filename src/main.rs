mod scanner;
mod token;

use std::{fs::File, io::prelude::Read, path::Path};

use scanner::Scanner;

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    println!("");

    for token in tokens {
        println!("{:?}", token)
    }
}

fn main() {
    let mut raw_src = String::new();

    File::open(Path::new("./lox-demo/main.lox"))
        .and_then(|mut file| file.read_to_string(&mut raw_src))
        .expect("Could not read file");

    run(raw_src)
}
