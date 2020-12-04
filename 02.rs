use std::str::FromStr;
use std::num::ParseIntError;
use std::fs;

struct PasswordRule {
    r_start: usize,
    r_end: usize,
    rule: char,
    password: String,
}

impl FromStr for PasswordRule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c==':' || c==' ' || c=='-').collect();

        Ok(PasswordRule {
            r_start: parts[0].parse().expect("start"),
            r_end: parts[1].parse().expect("end"),
            rule: parts[2].chars().next().expect("rule"), //Just the "first" (really only) char
            password: parts[4].to_string(),
        })
    }
}

impl PasswordRule {
    fn valid(&self) -> bool {
        let mut count: usize = 0;
        for token in self.password.chars() {
            if token==self.rule {count += 1}
        }

        count >= self.r_start && count <= self.r_end
    }

    fn valid_p2(&self) -> bool {
        let mut count: u32 = 0;
        let tokens: Vec<char> = self.password.chars().collect();
        
        if tokens[self.r_start-1]==self.rule {count += 1}
        if tokens[self.r_end-1]==self.rule {count += 1}

        count==1
    }
}

fn main() {
    let mut count: u32 = 0;
    for line in fs::read_to_string("02.in").unwrap().lines() {
        if line.parse::<PasswordRule>().expect("no nose").valid_p2() {count += 1}
    }

    println!("{}", count);
}
