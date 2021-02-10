#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize,
    m: usize,
    a: [usize; m],
  };
  let w: Vec<usize> = {
    let mut a = a;
    a.sort();
    let len = a.len();
    let mut w = vec![];
    let mut i = (0, true);
    let mut ai = 0;
    for t in 1..=n {
      if ai < len && a[ai] == t {
        i = (i.0 + if i.1 { 0 } else { 1 }, true);
        ai += 1;
      } else if i.1 {
        i.1 = false;
        w.push(1);
      } else {
        w[i.0] += 1;
      }
    }
    w
  };
  if let Some(&k) = w.iter().min() {
    let mut ans = 0;
    for t in w {
      ans += t / k + if 0 == t % k { 0 } else { 1 };
    }
    println!("{}", ans);
  } else {
    println!("0");
  }
}
