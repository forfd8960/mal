use std::io::{self, BufRead, Write};

fn main() -> anyhow::Result<()> {
    println!("Make a Lisp");

    let mut stdin = io::stdin().lock();
    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        println!(">> {}", buf);

        let trim_buf = buf.trim();
        if trim_buf == "q" || trim_buf == "quit" {
            break;
        }
    }
    Ok(())
}
