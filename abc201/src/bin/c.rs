fn main() {
  use proconio::marker::Chars;
  proconio::input! { s: Chars };
  let mut ans = 0;
  for n1 in 0..10 {
    for n2 in 0..10 {
      for n3 in 0..10 {
        'main: for n4 in 0..10 {
          for (n, c) in s.iter().enumerate() {
            match *c {
              'o' => {
                if n != n1 && n != n2 && n != n3 && n != n4 {
                  continue 'main;
                }
              }
              'x' => {
                if n == n1 || n == n2 || n == n3 || n == n4 {
                  continue 'main;
                }
              }
              '?' => {}
              _ => unreachable!(),
            }
          }
          ans += 1;
        }
      }
    }
  }
  println!("{}", ans);
}
