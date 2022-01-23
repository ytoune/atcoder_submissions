fn main() {
  proconio::input! { n: usize };
  let size = 2 * n - 1;
  let mut rels = vec![];
  for i in 0..size {
    let len = size - i;
    proconio::input! { line: [u64; len] };
    rels.push(line);
  }
  use fixedbitset::FixedBitSet;
  fn solve(rels: &Vec<Vec<u64>>, ans: u64, i: usize, unused: &FixedBitSet) -> u64 {
    rels[i]
      .iter()
      .cloned()
      .enumerate()
      .map(|(j, d)| (j + i + 1, d))
      .filter(|(j, _)| i != *j && unused.contains(*j))
      .map(|(j, d)| {
        let ans = ans ^ d;
        if let Some(t) = unused.ones().filter(|t| *t != j).next() {
          let mut unused = unused.clone();
          unused.set(j, false);
          unused.set(t, false);
          return solve(rels, ans, t, &unused);
        }
        ans
      })
      .max()
      .unwrap()
  }
  let mut unused = FixedBitSet::with_capacity(2 * n);
  for i in 0..(2 * n) {
    unused.insert(i);
  }
  unused.set(0, false);
  println!("{}", solve(&rels, 0, 0, &unused));
}
