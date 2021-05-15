fn main() {
  proconio::input! {
    mut mountains: [(String, usize)],
  };
  mountains.sort_by(|m, n| n.1.cmp(&m.1));
  println!("{}", mountains[1].0);
}
