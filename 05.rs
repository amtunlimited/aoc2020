// Notes from reading:
// F=0 B=1
// L=0 R=1
// Push chars to string at let from_str_radix do the rest

use std::str::FromStr;
use std::num::ParseIntError;
use std::fs;
use std::cmp;
use std::collections::HashSet;

struct SeatID(u32);

impl FromStr for SeatID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = String::new();

        for c in s.chars() {
            match c {
                'F' | 'L' => res.push('0'),
                'B' | 'R' => res.push('1'),
                i => res.push(i), // Will cause error in conversion 
                                  // below, didn't want to deal with
                                  // trying to roll my own ParseIntError`
            }
        }

        Ok(SeatID(u32::from_str_radix(res.as_str(), 2)?))
    }
}

fn main() {
    let mut max: u32 = 0;
    let mut min: u32 = 1000000000;
    let mut ids = HashSet::new();

    for line in fs::read_to_string("in/05.in").unwrap().lines() {
        let id = line.parse::<SeatID>().unwrap().0;
        max = cmp::max(max, id);
        min = cmp::min(min, id);
        ids.insert(id);
    }

    for id in min..max {
        if !ids.contains(&id) {
            println!("{}", id);
            break;
        }
    }
}
