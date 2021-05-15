fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    hsize: usize, wsize: usize,
    world: [Chars; hsize],
  };
  let get_cell = |h: usize, w: usize| {
    if '+' == world[h][w] {
      1
    } else {
      -1
    }
  };
  let mut scores: Vec<Vec<i32>> = vec![vec![0; wsize]; hsize];
  for h in (0..hsize).rev() {
    for w in (0..wsize).rev() {
      let is_tkhs = 1 == (h + w) % 2;
      let sign = if is_tkhs { 1 } else { -1 };
      let mut scr: Option<i32> = None;
      let calc = |scr: Option<i32>, h: usize, w: usize| {
        let s = scores[h][w] + sign * get_cell(h, w);
        scr
          .map(|t| if is_tkhs { t.max(s) } else { t.min(s) })
          .or_else(|| Some(s))
      };
      if h + 1 < hsize {
        scr = calc(scr, h + 1, w);
      }
      if w + 1 < wsize {
        scr = calc(scr, h, w + 1);
      }
      if let Some(s) = scr {
        scores[h][w] = s;
      }
    }
  }
  let score = scores[0][0];
  use std::cmp::Ordering;
  match score.cmp(&0) {
    Ordering::Equal => println!("Draw"),
    Ordering::Greater => println!("Aoki"),
    Ordering::Less => println!("Takahashi"),
  };
}
