fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    x: Chars,
    m: u128,
  };
  let us = x
    .iter()
    .map(|c| c.to_digit(10).unwrap() as u128)
    .collect::<Vec<_>>();
  let top: u128 = *us.iter().max().unwrap();
  if 1 == x.len() {
    println!("{}", if top as u128 <= m { "1" } else { "0" });
    return;
  }
  let sl = Solve::new(us, m);
  let c = top as u128 + 1;

  let mut base = c; // start;

  let mut max = c * 2;
  while sl.check(max).is_some() {
    max *= 2;
  }

  let mut size = max - c; // end - base;
  loop {
    let half = size / 2;
    let mid = base + half;
    if sl.check(mid).is_some() {
      base = mid;
    }
    if 0 == half {
      println!("{}", base + 1 - c);
      return;
    }
    size -= half;
  }
}

struct Solve {
  us: Vec<u128>,
  m: u128,
}

impl Solve {
  fn new(us: Vec<u128>, m: u128) -> Solve {
    Solve { us, m }
  }
  fn check(&self, c: u128) -> Option<u128> {
    let mut n = 0;
    for &u in self.us.iter() {
      n = n * c + u;
      if self.m < n {
        return None;
      }
    }
    Some(n)
  }
}
