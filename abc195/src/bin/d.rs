fn main() {
  use proconio::marker::Usize1;
  use std::cmp::Reverse;
  proconio::input! {
    n: usize,
    m: usize,
    q: usize,
    baggages: [(usize, usize); n],
    boxsizes: [usize; m],
    queries: [(Usize1, Usize1); q],
  };
  let mut baggages = baggages;
  baggages.sort_by_key(|v| (Reverse(v.1), v.0));
  let mut boxsizes: Vec<_> = boxsizes.iter().copied().enumerate().collect();
  boxsizes.sort_by_key(|v| (v.1, v.0));
  for query in queries.iter() {
    let mut ans = 0;
    let mut used = vec![false; m];
    'main: for bag in baggages.iter() {
      for bx in boxsizes.iter() {
        if (bx.0 < query.0 || query.1 < bx.0) && bag.0 <= bx.1 && !used[bx.0] {
          used[bx.0] = true;
          ans += bag.1;
          continue 'main;
        }
      }
    }
    println!("{}", ans);
  }
}
