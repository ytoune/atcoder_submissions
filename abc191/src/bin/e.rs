use proconio::input;
use std::collections::*;

type RoadsMap = HashMap<usize, Vec<(usize, u32)>>;

struct Solver {
  n: usize,
  r: RoadsMap,
}

impl Solver {
  fn new(n: usize, r: Vec<(usize, usize, u32)>) -> Solver {
    let mut r_: RoadsMap = HashMap::new();
    for (a, b, c) in r {
      let e = r_.entry(a - 1).or_default();
      e.push((b - 1, c));
    }
    Solver { n, r: r_ }
  }
  fn cost(&self, town: usize) -> Option<u32> {
    use std::cmp::Reverse;
    let r = &self.r;
    let mut tasks: BinaryHeap<(Reverse<u32>, usize)> = BinaryHeap::new();
    let mut done = vec![false; self.n];
    if let Some(r) = r.get(&town) {
      for &(b, c) in r {
        tasks.push((Reverse(c), b));
      }
    }
    while let Some((Reverse(cost), tw)) = tasks.pop() {
      if tw == town {
        return Some(cost);
      }
      if done[tw] {
        continue;
      }
      done[tw] = true;
      if let Some(r) = r.get(&tw) {
        for &(b, c) in r.iter().filter(|r| !done[r.0]) {
          tasks.push((Reverse(cost + c), b));
        }
      }
    }
    None
  }
  fn solve(&self) {
    use std::io::{stdout, BufWriter, Write};
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for tw in 0..self.n {
      if let Some(cost) = self.cost(tw) {
        write!(out, "{}\n", cost).unwrap();
      } else {
        let _ = out.write(b"-1\n").unwrap();
      }
    }
  }
}

fn main() {
  input! {
    n: usize,
    m: usize,
    r: [(usize, usize, u32); m],
  };
  let sol = Solver::new(n, r);
  sol.solve();
}
