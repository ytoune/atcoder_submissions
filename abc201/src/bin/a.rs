fn main() {
  proconio::input! { mut a: [usize; 3] };
  a.sort();
  if a[1] - a[0] == a[2] - a[1] {
    println!("Yes");
  } else {
    println!("No");
  }
}
