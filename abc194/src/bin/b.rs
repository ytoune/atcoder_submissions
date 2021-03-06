fn main() {
  proconio::input! {
    n: usize,
    workers: [(usize, usize); n],
  };
  let workers: Vec<(usize, (usize, usize))> = workers.into_iter().enumerate().collect();
  let need_a: Vec<(usize, usize)> = workers.iter().map(|(i, p)| (p.0, *i)).collect();
  let need_b: Vec<(usize, usize)> = workers.iter().map(|(i, p)| (p.1, *i)).collect();
  let a_1 = need_a.iter().min().unwrap();
  let a_2 = need_a.iter().filter(|v| a_1.1 != v.1).min().unwrap();
  let b_1 = need_b.iter().min().unwrap();
  let b_2 = need_b.iter().filter(|v| b_1.1 != v.1).min().unwrap();
  let list = [(a_1, b_1), (a_2, b_1), (a_1, b_2), (a_2, b_2)];
  let mut min: Option<usize> = None;
  for &(a, b) in list.iter() {
    let tm = if a.1 == b.1 { a.0 + b.0 } else { a.0.max(b.0) };
    min = min.map(|t| t.min(tm)).or_else(|| Some(tm));
  }
  println!("{}", min.unwrap());
}
