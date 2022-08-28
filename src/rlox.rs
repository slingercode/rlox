use std::{fs, io::{self, Write}};

pub fn run_file(path: String) {
    let content = fs::read_to_string(path)
        .expect("You need to provide a valid source file");

    run(content);
}

pub fn run_promt() -> io::Result<()> {
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

        run(input);
        buf.clear();
    }

    return Ok(());
}

fn run(source: String) {
    print!("{}", source);
}
