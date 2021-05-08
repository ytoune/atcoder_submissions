fn main() {
  proconio::input! {
    n: usize,
    a: [u64; n],
  };
  use std::collections::*;
  let mut map = HashMap::<u64, Vec<usize>>::new();
  let mut st = PowerPermutations::new(2, n);
  st.next();
  while let Some(st) = st.next() {
    let sum = st
      .iter()
      .zip(&a)
      .map(|(s, a)| if *s == 1 { *a % 200 } else { 0 })
      .sum::<u64>()
      % 200;
    if let Some(st2) = map.get(&sum) {
      let print_nums = |st: &[usize]| {
        let nums = st
          .iter()
          .enumerate()
          .filter(|(_, u)| **u == 1)
          .map(|(i, _)| format!("{}", i + 1))
          .collect::<Vec<_>>();
        println!("{} {}", nums.len(), nums.join(" "));
      };
      println!("Yes");
      print_nums(&st2);
      print_nums(&st);
      return;
    } else {
      map.insert(sum, st.iter().copied().collect());
    }
  }
  println!("No");
}

struct PowerPermutations {
  count: usize,
  state: Vec<usize>,
  run: bool,
}
impl PowerPermutations {
  pub fn new(count: usize, size: usize) -> Self {
    PowerPermutations {
      count,
      state: vec![0; size],
      run: false,
    }
  }
  fn next(&mut self) -> Option<&[usize]> {
    if self.run {
      for s in self.state.iter_mut() {
        if *s < self.count - 1 {
          *s += 1;
          return Some(&self.state);
        }
        *s = 0;
      }
      self.run = false;
      None
    } else {
      self.run = true;
      Some(&self.state)
    }
  }
}
