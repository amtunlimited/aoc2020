use std::fs;
use std::collections::HashSet;

fn main() {
  let mut ledger = HashSet::<i32>::new();
  for line in fs::read_to_string("01.in").unwrap().lines() {
    let number: i32 = line.parse().unwrap();
    if ledger.contains(&(2020 - number)) {
        println!("{}", number * (2020 - number));
        break;
    }

    ledger.insert(number);
  }
}
