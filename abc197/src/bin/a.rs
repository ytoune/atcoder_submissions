fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    list: Chars,
  };
  let mut list: Vec<_> = list.into_iter().collect();
  list.rotate_left(1);
  let list: String = list.into_iter().collect();
  println!("{}", list);
}
