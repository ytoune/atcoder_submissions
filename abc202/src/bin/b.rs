fn main() {
  use proconio::marker::Chars;
  proconio::input! { s: Chars };
  let s: String = s
    .into_iter()
    .rev()
    .map(|c| match c {
      '0' => '0',
      '1' => '1',
      '6' => '9',
      '8' => '8',
      '9' => '6',
      _ => unreachable!(),
    })
    .collect();
  println!("{}", s);
}
