fn main() {
  proconio::input! { houses: [(i64, i64)] };
  use itertools::Itertools;

  let mut x_ls: Vec<_> = houses.iter().map(|h| h.0).collect();
  let mut y_ls: Vec<_> = houses.iter().map(|h| h.1).collect();
  x_ls.sort();
  y_ls.sort();

  let x_ls2: [i64; 4] = [x_ls[0], x_ls[1], x_ls[x_ls.len() - 2], x_ls[x_ls.len() - 1]];
  let y_ls2: [i64; 4] = [y_ls[0], y_ls[1], y_ls[y_ls.len() - 2], y_ls[y_ls.len() - 1]];

  let mut x_cnt: [usize; 4] = [0; 4];
  let mut y_cnt: [usize; 4] = [0; 4];

  let houses: Vec<_> = houses
    .iter()
    .filter(|h| {
      if let Some(xi) = x_ls2
        .iter()
        .position(|x| *x == h.0)
        .filter(|xi| x_cnt[*xi] < 2)
      {
        x_cnt[xi] += 1;
        return true;
      }
      if let Some(yi) = y_ls2
        .iter()
        .position(|y| *y == h.1)
        .filter(|yi| y_cnt[*yi] < 2)
      {
        y_cnt[yi] += 1;
        return true;
      }
      false
    })
    .collect();

  let mut list: Vec<_> = houses
    .iter()
    .combinations(2)
    .map(|p| {
      let a = p[0];
      let b = p[1];
      (a.0 - b.0).abs().max((a.1 - b.1).abs())
    })
    .collect();
  list.sort();
  if let Some(v) = list.get(list.len() - 2) {
    println!("{}", v);
  } else {
    println!("0");
  }
}
