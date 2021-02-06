// use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: i64,
    x: i64,
    a: [i64; n]
  };
  let b = a
    .into_iter()
    .filter(|&u| u != x)
    .map(|u| format!("{}", u))
    .collect::<Vec<String>>()
    .join(" ");
  println!("{}", b);
}
