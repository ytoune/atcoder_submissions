#![allow(dead_code)]

use proconio::input;

/**
 * 参考
 * https://qiita.com/Tukaene/items/63b86d918e4924c12d36
 */

struct Solver {
  x: i64,
  y: i64,
  r: i64,
}

#[inline]
fn cf(x: i64, r: i64) -> (i64, i64) {
  fn ceil(n: i64) -> i64 {
    let t = n - n % 10000;
    if n <= t {
      t
    } else {
      t + 10000
    }
  }
  fn floor(n: i64) -> i64 {
    let t = n - n % 10000;
    if t <= n {
      t
    } else {
      t - 10000
    }
  }
  let low = ceil(x - r);
  let high = floor(x + r);
  (low, high)
}

impl Solver {
  fn solve(self) -> i64 {
    let mut num = 0;
    let x = self.x;
    let y = self.y;
    let r = self.r;
    let (start, end) = cf(x, r);
    // println!("{} {}", start, end);
    let mut i = start;
    loop {
      if i > end {
        break;
      }
      let p = ((r.pow(2) - (x - i).pow(2)) as f64).sqrt() as i64;
      let (bottom, top) = cf(y, p);
      let mut j = bottom;
      loop {
        if j > top {
          break;
        }
        num += 1;
        j += 10000;
      }
      i += 10000;
    }
    num
  }
}

fn main() {
  input! {
    x: f64,
    y: f64,
    r: f64,
  };
  let x = (x * 10000.0) as i64;
  let y = (y * 10000.0) as i64;
  let r = (r * 10000.0) as i64;
  let s = Solver { x, y, r };
  println!("{}", s.solve());
}
