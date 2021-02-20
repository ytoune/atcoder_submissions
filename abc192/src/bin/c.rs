fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    n: Chars,
    k: usize,
  };
  let n = n;
  let mut n = n;
  for _ in 1..=k {
    n = getnext(n);
  }
  println!("{}", n.into_iter().collect::<String>());
}

fn getnext(n: Vec<char>) -> Vec<char> {
  let mut n1 = n;
  n1.sort();
  let n2 = n1.clone();
  n1.reverse();
  let n1 = n1.into_iter().collect::<String>().parse::<usize>().unwrap();
  let n2 = n2.into_iter().collect::<String>().parse::<usize>().unwrap();
  let g = n1 - n2;
  format!("{}", g).chars().collect()
}
