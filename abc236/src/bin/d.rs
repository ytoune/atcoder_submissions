fn main() {
  proconio::input! { n: usize };
  let size = 2 * n - 1;
  let mut rels = vec![];
  for i in 0..size {
    let len = size - i;
    proconio::input! { line: [u64; len] };
    rels.push(line);
  }
  use std::collections::HashSet;
  fn solve(rels: &Vec<Vec<u64>>, ans: u64, i: usize, used: &HashSet<usize>) -> u64 {
    rels[i]
      .iter().cloned().enumerate()
      .map(|(j, d)| (j + i + 1, d))
      .filter(|(j, _)| i != *j && !used.contains(j))
      .map(|(j, d)| { 
        let ans = ans ^ d;
        if i + 1 < rels.len() {
          let mut used = used.clone();
          used.insert(j);
          for t in (i + 1)..rels.len() {
            if !used.contains(&t) {
              used.insert(t);
              return solve(rels, ans, t, &used);
            }
          }
        }
        ans
      })
      .max().unwrap()
  }
  println!("{}", solve(&rels, 0, 0, &[0].iter().cloned().collect()));
}
