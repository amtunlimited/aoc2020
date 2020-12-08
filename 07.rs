// An attempt at a grammer for this problem is as follows:
// NOTE: For this we assume tokens are seperated by white space
// rule := bag-token " bags contain " bag-list "."
// bag-list := bag *(", " bag)
// bag := single-bag / multi-bag
// single-bag := "1 " bag-token " bag"
// multi-bag := GT1 " " bag-token " bags"
// bag-token := ADJECTIVE " " COLOR
//
// Where GT1 is a number greater than one

use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone)]
struct BagCount(u32, String);

struct BagRule {
    desc: String,
    contains: Vec<BagCount>,
}

struct BagBiGraph {
    // What each bag can hold
    fwd: HashMap<String, Vec<BagCount>>,
    // What can hold each bag
    back: HashMap<String, Vec<BagCount>>,
}

impl FromStr for BagRule {
    type Err = String;

    fn from_str(s: &str) ->Result<Self, Self::Err> {
         // Taken directly from above
         let mut tokens = s.split_ascii_whitespace();
         let desc = format!("{} {}",tokens.next().unwrap(), tokens.next().unwrap());
         
         // eat "bags contain"
         tokens.next().unwrap();
         tokens.next().unwrap();

         let mut contains = Vec::new();

         let bag = tokens.next().unwrap();

         if let "no" = bag {return Ok(BagRule {desc, contains})}
         // I could write a separate from str for BagCount but nah...
         contains.push(BagCount(bag.parse().unwrap(), format!("{} {}",tokens.next().unwrap(), tokens.next().unwrap())));

         // Checks for the end of the loop AND eats the "bag" token
         while tokens.next().unwrap().chars().last().unwrap() != '.' {
            contains.push(BagCount(tokens.next().unwrap().parse().unwrap(), format!("{} {}",tokens.next().unwrap(), tokens.next().unwrap())));
         }

         Ok(BagRule {desc, contains})
    }
}

impl FromStr for BagBiGraph {
    type Err = String;

    fn from_str(s: &str) ->Result<Self, Self::Err> {
        let mut fwd = HashMap::new();
        let mut back = HashMap::new();
        for line in s.lines() {
            let rule = line.parse::<BagRule>().unwrap();

            fwd.insert(rule.desc.clone(), rule.contains.clone());

            for bag_count in rule.contains {
                match back.get_mut(&bag_count.1) {
                    None => {
                        let mut contains = Vec::new();
                        contains.push(BagCount(bag_count.0, rule.desc.clone()));
                        back.insert(bag_count.1, contains);
                    },
                    Some(contains) => {
                        contains.push(BagCount(bag_count.0, rule.desc.clone()));
                    }
                }
            }
        }

        Ok(BagBiGraph {fwd, back})
    }
}

impl BagBiGraph {
    fn part1(&self, start: &str) -> usize {
        let mut visited: HashSet<String> = HashSet::new();
        let mut stack = Vec::new();

        stack.push(start.to_string());

        while !stack.is_empty() {
            let curr = stack.pop().unwrap();

            visited.insert(curr.to_string());

            let next = self.back.get(curr.as_str());

            if let None = next {continue;}

            for bag in next.unwrap() {
                if !visited.contains(&bag.1) {
                    stack.push(bag.1.clone());
                }
            }
        }

        visited.len()
    }
    /*
     * Left as a lesson...
    fn part2(&self, start: &str) -> u32 {
        let mut visited: HashSet<String> = HashSet::new();
        let mut count: HashMap<String, u32> = HashMap::new();
        let mut stack: Vec<String> = Vec::new();

        stack.push(start.to_string());
        count.insert(start.to_string(), 1);

        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            let curr_count: u32 = *count.get(&curr).unwrap();

            visited.insert(curr.to_string());

            let next = self.fwd.get(curr.as_str());

            if let None = next {continue;}

            for bag in next.unwrap() {
                //if !visited.contains(&bag.1) {
                    stack.push(bag.1.clone());
                    let result: u32 = *count.get(&bag.1).unwrap_or(&0);
                    count.insert(bag.1.clone(), (bag.0 * curr_count) + result);
                //}
            }
        }

        count.values().sum::<u32>() - 1
    }
    */
    fn part2(&self, start: &str) -> u32 {
        let mut queue: VecDeque<BagCount> = VecDeque::new();

        queue.push_back(BagCount(1, start.to_string()));

        let mut count = 0;

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            count += curr.0;

            for next in self.fwd.get(&curr.1).unwrap() {
                queue.push_back(BagCount(curr.0 * next.0, next.1.clone()));
            }
        }

        count - 1
    }
}

fn main() {
    let rules = fs::read_to_string("in/07.in").unwrap().parse::<BagBiGraph>().unwrap();

    println!("{}", rules.part2("shiny gold"));
}
