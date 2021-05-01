fn main() {
  proconio::input! {
    n: usize,
    mems: [[u64; 5]; n],
  };
  fn calc_score(mems: &[&Vec<u64>]) -> u64 {
    (0..5)
      .map(|t| mems.iter().map(|m| m[t]).max().unwrap())
      .min()
      .unwrap()
  }
  use itertools::Itertools;
  let s = mems
    .iter()
    .combinations(3)
    .map(|select| calc_score(&select))
    .max()
    .unwrap();
  println!("{}", s);
}
