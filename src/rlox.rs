use std::{fs, io::{self, Write}, process};

use crate::scanner::Scanner;

pub(crate) struct Rlox {
    had_error: bool,
}

impl Rlox {
    pub(crate) fn new() -> Self {
        Self { had_error: false }
    }

    pub(crate) fn run_file(&mut self, path: String) {
        let content = fs::read_to_string(path)
            .expect("You need to provide a valid source file");
        
        self.run(content);

        if self.had_error {
            process::exit(0);
        }
    }

    pub(crate) fn run_promt(&mut self) -> io::Result<()> {
        let mut buf = String::new();
        let end_word = String::from("exit");
    
        println!("rlox interpreter\n");
    
        loop {
            print!("> ");
    
            io::stdout().flush()?;
            io::stdin().read_line(&mut buf)?;
    
            let input = buf.trim().to_string();
    
            if input.eq(&end_word) {
                break;
            }
    
            self.run(input);
            self.had_error = false;

            buf.clear();
        }
    
        return Ok(());
    }

    fn run(&mut self, source: String) {
        let mut scanner_instance = Scanner::new(source);

        let tokens = scanner_instance.scan_tokens().iter();

        for token in tokens {
            print!("Token {:?}", token.token_type);
            print!(" / Literal \"{}\"", token.literal.string);
            print!(" / Num {}\n", token.literal.number);
        }
    }

    // pub fn error(&mut self, line: u32, message: &str) {
    //     self.report(line, "", message);
    // }

    // fn report(&mut self, line: u32, on: &str, message: &str) {
    //     println!("[line {}] on {}: {}", line, on, message);
    //     self.had_error = true;
    // }
}
