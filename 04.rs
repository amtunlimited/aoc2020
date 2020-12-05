use std::fs;
use std::collections::HashMap;
use std::str::FromStr;

struct Passport(HashMap<String, String>);

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = HashMap::new();

        for item in s.split_ascii_whitespace() {
            let tokens: Vec<&str> = item.split(':').collect();
            passport.insert(tokens[0].to_string(), tokens[1].to_string());
        }

        Ok(Passport(passport))
    }
}

impl Passport {
    fn validator(key: &str, s: &str) -> bool {
        match key {
            "byr" => {
                match s.parse::<u32>() {
                    Err(_) => false,
                    Ok(i) => i >= 1920 && i <= 2002
                }
            },
            "iyr" => {
                match s.parse::<u32>() {
                    Err(_) => false,
                    Ok(i) => i >= 2010 && i <= 2020
                }
            },
            "eyr" => {
                match s.parse::<u32>() {
                    Err(_) => false,
                    Ok(i) => i >= 2020 && i <= 2030
                }
            },
            "hgt" => {
                let parts = s.split_at(s.len()-2);

                match parts.0.parse::<u32>() {
                    Err(_) => false,
                    Ok(i) => {
                        match parts.1 {
                            "in" => i >= 59 && i <= 76,
                            "cm" => i >= 150 && i <= 193,
                            _ => false,
                        }
                    }
                }
            },
            "hcl" => {
                let parts = s.split_at(1);
                if parts.1.len() != 6 {return false}

                match parts.0 {
                    "#" => u64::from_str_radix(parts.1, 16).is_ok(),
                    _ => false,
                }
            },
            "ecl" => {
                match s {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false
                }
            },
            "pid" => {
                s.len() == 9 && s.parse::<u32>().is_ok()
            },
            _ => false,
            //"cid", Optional
        }
    }

    fn validate(&self) -> bool {
        let keys = [
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
        ];

        for key in &keys {
            match self.0.get(&key.to_string()) {
                None => return false,
                Some(s) => {if !Self::validator(key, s) {println!("{}", key); return false}}
            }
        }

        true
    }
}

fn main() {
    let mut count = 0;
    for item in fs::read_to_string("in/04.in").unwrap().split("\n\n") {
        if item.parse::<Passport>().unwrap().validate() {
            count += 1;
        }
    }

    println!("{}", count);
}
