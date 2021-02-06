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
      let e = r_.entry(a).or_default();
      e.push((b, c));
    }
    Solver { n, r: r_ }
  }
  fn cost(&self, town: usize) -> Option<u32> {
    use std::cmp::Reverse;
    let r = &self.r;
    let mut tasks: BinaryHeap<(Reverse<u32>, usize)> = BinaryHeap::new();
    let mut done: HashSet<usize> = HashSet::new();
    tasks.push((Reverse(0), town));
    'main: loop {
      match tasks.pop() {
        None => {
          return None;
        }
        Some((Reverse(cost), tw)) => {
          if tw == town {
            if 0 != cost {
              return Some(cost);
            }
          } else {
            if done.contains(&tw) {
              continue 'main;
            }
            done.insert(tw);
          }
          if let Some(r) = r.get(&tw) {
            for (b, c) in r {
              if done.contains(b) {
                continue;
              }
              let cost = cost + c;
              tasks.push((Reverse(cost), *b));
            }
          }
        }
      }
    }
  }
  fn solve(&self) {
    use std::io::{stdout, BufWriter, Write};
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for tw in 1..=self.n {
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
