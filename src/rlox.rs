use std::{fs, io::{self, Write}, process};

pub struct Rlox {
    pub had_error: bool,
}

impl Rlox {
    pub fn run_file(&self, path: String) {
        let content = fs::read_to_string(path)
            .expect("You need to provide a valid source file");
        
        self.run(content);

        if self.had_error {
            process::exit(0);
        }
    }

    pub fn run_promt(&mut self) -> io::Result<()> {
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

    fn run(&self, source: String) {
        println!("{}", source);
    }

    fn error(&mut self, line: u32, message: String) {
        self.report(line, String::new(), message);
    }

    fn report(&mut self, line: u32, on: String, message: String) {
        println!("[line {}] on {}: {}", line, on, message);
        self.had_error = true;
    }
}
