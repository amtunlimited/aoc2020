use std::collections::HashSet;
use std::str::FromStr;
use std::fs;

struct AnswerGroup(Vec<HashSet<char>>);

impl FromStr for AnswerGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut answers = Vec::new();

        for line in s.lines() {
            let mut group = HashSet::new();
            for answer in line.chars() {
                if answer.is_alphabetic() {
                    group.insert(answer);
                }
            }

            answers.push(group);
        }

        Ok(AnswerGroup(answers))
    }
}

impl AnswerGroup {
    fn uniq_part_1(&self) -> usize {
        self.0.iter().fold(HashSet::new(), |a, x| a.union(x).cloned().collect()).len()
    }

    fn uniq_part_2(&self) -> usize {
        self.0.iter().fold(self.0[0].clone(), |a, x| a.intersection(x).cloned().collect()).len()
    }
}

fn main() {
    let mut count = 0;
    for group in fs::read_to_string("in/06.in").unwrap().split("\n\n") {
        let uniq = group.parse::<AnswerGroup>().unwrap().uniq_part_2(); 
        count += uniq;
    }

    println!("{}", count);
}
