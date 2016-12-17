use std::io::{stdin, stdout, stderr, Read, Write};
use std::io;
use std::process::exit;

fn has_data(size: usize) -> Option<usize> {
    match size {
        0 => None,
        x => Some(x),
    }
}

fn run() -> io::Result<()> {
    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut buf = [0; 4096];

    while let Some(size) = stdin.read(&mut buf).map(has_data)? {
        let mut flush = false;

        for b in &mut buf[0..size] {
            if *b == b'\r' {
                *b = b'\n';
                flush = true;
            }
        }

        stdout.write(&buf[0..size])?;
        if flush {
            stdout.flush()?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        let _ = write!(stderr(), "Error: {}", e);
        exit(1);
    }
}
