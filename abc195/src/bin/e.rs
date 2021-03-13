fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    n: usize,
    s: Chars,
    x: Chars,
  };
  let s: Vec<_> = s.iter().map(|c| c.to_digit(10).unwrap()).collect();
  if 0 != *s.last().unwrap() && 7 != *s.last().unwrap() && 'A' == *x.last().unwrap() {
    println!("Aoki");
    return;
  }
  // let dmax = 2u32.pow(n as u32) as usize;
  // let mut map: Vec<_> = vec![0; dmax];
  // for d in 0..dmax {
  //   let mut num = 0;
  //   let mut i = 1;
  //   for (i, v) in s.iter().copied().enumerate() {

  //   }
  // }
}
