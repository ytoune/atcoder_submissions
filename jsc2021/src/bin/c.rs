fn main() {
  proconio::input! {
    a: usize, b: usize,
  };
  println!("{}", find_max_gcd(a, b));
}

/// https://www.geeksforgeeks.org/find-pair-maximum-gcd-array/
///
/// function to find
/// GCD of pair with
/// max GCD in the
/// array
fn find_max_gcd(start: usize, end: usize) -> usize {
  // Calculating MAX in array
  let mut high = 0;
  for a in start..=end {
    high = high.max(a);
  }

  // Maintaining count array
  let mut count: Vec<usize> = vec![0; high + 1];
  for a in start..=end {
    count[a] += 1;
  }

  // Iterating from MAX to 1
  // GCD is always between
  // MAX and 1. The first
  // GCD found will be the
  // highest as we are
  // decrementing the potential
  // GCD
  for i in (1..=high).rev() {
    let mut j = i;

    // Variable to store the
    // multiples of a number
    let mut counter = 0;

    // Iterating from current
    // potential GCD
    // till it is less than
    // MAX
    while j <= high {
      // A multiple found

      if count[j] >= 2 {
        return j;
      } else if count[j] == 1 {
        counter += 1;
      }

      // Incrementing potential
      // GCD by itself
      // To check i, 2i, 3i....
      j += i;

      // 2 multiples found,
      // max GCD found
      if counter == 2 {
        return i;
      }
    }
  }
  0
}
