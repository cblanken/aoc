use aoc_utils::read_file;
use std::fmt;
use std::cmp::max;



struct CubeSet<'a> {
    color: &'a str,
    count: u16,
}

impl fmt::Display for CubeSet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {}, {} ]", self.count, self.color)
    }
}

struct GameRound<'a> {
    id: u32,
    cube_groups: Vec<Vec<CubeSet<'a>>>,
}

impl fmt::Display for GameRound<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("ID: {} ", self.id);
        for group in self.cube_groups.iter() {
            for cubeset in group {
                print!("{} ", cubeset);
            }
        }

        write!(f, "")
    }
}

fn minimum_cubeset(cubegroups: Vec<Vec<CubeSet>>) -> [u16; 3] {
    let mut min_counts = [0; 3]; // red, green, blue
    for cubegroup in cubegroups {
        for cubeset in cubegroup {
            if cubeset.color == "red" {
                min_counts[0] = max(min_counts[0], cubeset.count);
            } else if cubeset.color == "green" {
                min_counts[1] = max(min_counts[1], cubeset.count);
            } else if cubeset.color == "blue" {
                min_counts[2] = max(min_counts[2], cubeset.count);
            }
        }
    }

    min_counts
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);
    let rounds = data.lines().into_iter()
        .map(|line| {
            let split = line.split(":").collect::<Vec<&str>>();
            let id = split[0].split(" ").collect::<Vec<&str>>()[1].trim();

            let mut cube_groups: Vec::<Vec::<CubeSet>> = Vec::new();
            for cubeset in split[1].split(";") {
                let mut cubesets: Vec::<CubeSet> = Vec::new();
                let cubes = cubeset.split(",");
                for cube in cubes.into_iter() {
                    let cube_split: Vec::<&str> = cube.trim().split(" ").collect();
                    let count = cube_split[0].trim().parse::<u16>().unwrap();
                    let color = cube_split[1].trim();
                    cubesets.push(CubeSet { color, count });
                }

                cube_groups.push(cubesets);
            }

            GameRound { 
                id: id.parse::<u32>().unwrap(),
                cube_groups,
            }
        });

    rounds.fold(0, |acc, round| {
        let power = minimum_cubeset(round.cube_groups)
            .into_iter()
            .fold(1, |acc, count| { count * acc });

        return acc as u32 + power as u32;
    }).to_string()
}
