use std::fs;
use std::str::FromStr;

struct CylinderMap {
    width: usize,
    height: usize,
    map: Vec<Vec<bool>>,
}

impl FromStr for CylinderMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let width: usize = lines[0].len();
        let height: usize = lines.len();
        let mut map = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();

            for tile in line.chars() {
                row.push(match tile {
                    '#' => true,
                    '.' => false,
                    i => {return Err(format!("Wrong char: {}", i))},
                });
            }

            map.push(row);
        }

        Ok(CylinderMap {width, height, map})
    }
}

impl CylinderMap {
    fn traverse(&self, x_vel: usize, y_vel: usize) -> u32 {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;

        while y<self.height {
            if self.map[y][x] {count += 1};

            x = (x +  x_vel) % self.width;
            y += y_vel;
        }

        count
    }
}

fn main() {
    let mut count = 1;
    let map = fs::read_to_string("03.in").unwrap().parse::<CylinderMap>().unwrap();
    let slopes = [
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    ];

    for slope in &slopes {
        count *= map.traverse(slope.0, slope.1);
    }
    println!("{}", count);
}
