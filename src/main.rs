extern crate memchr;

use std::io::{self, Read, Write};
use std::process::exit;
use memchr::memchr;

struct NewlineWrapper<R: Read> {
    inner: R
}

impl<R: Read> NewlineWrapper<R> {
    fn new(inner: R) -> Self {
        NewlineWrapper {
            inner: inner,
        }
    }
}

impl<R: Read> Read for NewlineWrapper<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.read(buf)?;
        let mut start = 0;
        while start < size {
            let to_check = &mut buf[start..size];
            match memchr(b'\r', to_check) {
                Some(i) => {
                    to_check[i] = b'\n';
                    start += i + 1;
                },
                None => break,
            }
        }
        Ok(size)
    }
}

fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut stdin = NewlineWrapper::new(stdin);
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = io::LineWriter::new(stdout);

    io::copy(&mut stdin, &mut stdout).map(|_| ())
}

fn main() {
    if let Err(e) = run() {
        let _ = write!(io::stderr(), "Error: {}", e);
        exit(1);
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use super::NewlineWrapper;

    fn replacement_test(input: &[u8]) {
        let mut expected_output = input.to_vec();
        for ch in expected_output.iter_mut() {
            if (*ch) == b'\r' {
                *ch = b'\n';
            }
        }

        let mut output = vec![0u8; input.len()];
        let mut input = NewlineWrapper::new(input);
        input.read_exact(&mut output).unwrap();
        assert_eq!(&output[..], &expected_output[..]);
    }

    #[test]
    fn last_char_lf() {
        replacement_test(b"hi there\r");
    }

    #[test]
    fn multiple_lf() {
        replacement_test(b"\r\r\rword\rmiddle\r\rrest");
    }
}
