use std::{fmt::Debug, io::BufRead, str::FromStr};

/// Byte based scanner for reading input
pub struct Scanner<R: BufRead> {
  reader: R,
  buf: Vec<u8>,
  pos: usize,
}

/// Scanner that reads from standard input.
impl Scanner<std::io::StdinLock<'static>> {
  pub fn mk_from_stdin() -> Self {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    Self::new(lock)
  }
}

impl<R: BufRead> Scanner<R> {
  pub fn new(reader: R) -> Self {
    Self {
      reader,
      buf: Vec::with_capacity(1 << 16),
      pos: 0,
    }
  }

  fn next_item<T>(&mut self) -> Option<T>
  where
    T: FromStr,
    <T as FromStr>::Err: Debug,
  {
    loop {
      while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
        self.pos += 1;
      }

      if self.pos < self.buf.len() {
        let start = self.pos;
        while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
          self.pos += 1;
        }
        let token =
          std::str::from_utf8(&self.buf[start..self.pos]).expect("Input token is not valid UTF-8");
        return Some(
          token
            .parse::<T>()
            .expect("Failed to parse token from input"),
        );
      }

      self.buf.clear();
      self.pos = 0;
      let read = self
        .reader
        .read_until(b'\n', &mut self.buf)
        .expect("Failed to read line from input");
      if read == 0 {
        return None;
      }
    }
  }

  pub fn read_one<T>(&mut self) -> T
  where
    T: FromStr,
    <T as FromStr>::Err: Debug,
  {
    self.next_item().expect("Expected one token but found EOF")
  }

  pub fn read_array<T, const N: usize>(&mut self) -> [T; N]
  where
    T: FromStr,
    <T as FromStr>::Err: Debug,
  {
    std::array::from_fn(|_| self.read_one())
  }

  pub fn read_ints<const N: usize>(&mut self) -> [isize; N] {
    self.read_array()
  }

  pub fn read_int(&mut self) -> isize {
    self.read_one()
  }

  pub fn read_floats<const N: usize>(&mut self) -> [f64; N] {
    self.read_array()
  }

  pub fn read_float(&mut self) -> f64 {
    self.read_one()
  }

  pub fn read_words<const N: usize>(&mut self) -> [String; N] {
    self.read_array()
  }

  pub fn read_word(&mut self) -> String {
    self.read_one()
  }

  pub fn read_line(&mut self) -> String {
    let mut line = String::new();
    self
      .reader
      .read_line(&mut line)
      .expect("Failed to read line from input");
    line.trim_end().to_string()
  }

  pub fn read_to_string(&mut self) -> String {
    let mut s = String::new();
    self
      .reader
      .read_to_string(&mut s)
      .expect("Failed to read remaining input");
    s
  }
}

// === main functions for solving the problem ===
fn main() {
  let input = Scanner::mk_from_stdin();
  let mut stdout = std::io::stdout();
  solve(input, &mut stdout)
}

fn solve(mut inp: Scanner<impl BufRead>, out: &mut impl std::io::Write) {
}

// === tests ===
