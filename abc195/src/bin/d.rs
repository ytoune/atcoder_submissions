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
  }
  let baggages = {
    let mut baggages = baggages;
    baggages.sort_by_key(|v| (Reverse(v.1), v.0));
    baggages
  };
  for query in queries.iter() {
    let mut ans = 0;
    let mut boxes = usable_boxes(&boxsizes, query);
    boxes.sort();
    'main: for bag in baggages.iter() {
      for bx in boxes.iter_mut() {
        if bag.0 <= bx.0 && !bx.1 {
          bx.1 = true;
          ans += bag.1;
          continue 'main;
        }
      }
    }
    println!("{}", ans);
  }
}

fn usable_boxes(boxsizes: &[usize], query: &(usize, usize)) -> Vec<(usize, bool)> {
  boxsizes
    .iter()
    .copied()
    .enumerate()
    .filter(|(i, _)| *i < query.0 || query.1 < *i)
    .map(|(_, v)| (v, false))
    .collect()
}
