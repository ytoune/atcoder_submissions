fn main() {
  proconio::input! {
    n: usize,
    workers: [(usize, usize); n],
  };
  let workers: Vec<(usize, (usize, usize))> = workers.into_iter().enumerate().collect();
  let mut need_a: Vec<(usize, usize)> = workers.iter().map(|(i, p)| (p.0, *i)).collect();
  let mut need_b: Vec<(usize, usize)> = workers.iter().map(|(i, p)| (p.1, *i)).collect();
  need_a.sort();
  need_b.sort();
  let list = [
    (need_a[0], need_b[0]),
    (need_a[1], need_b[0]),
    (need_a[0], need_b[1]),
    (need_a[1], need_b[1]),
  ];
  let mut min: Option<usize> = None;
  for &(a, b) in list.iter() {
    let tm = if a.1 == b.1 { a.0 + b.0 } else { a.0.max(b.0) };
    min = min.map(|t| t.min(tm)).or_else(|| Some(tm));
  }
  println!("{}", min.unwrap());
}
