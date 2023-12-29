use aoc_utils::read_file;
use std::fmt;



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

fn check_cubeset(cubeset: CubeSet) -> bool {
    let bag: Vec::<CubeSet> = vec![
        CubeSet { color: "red", count: 12 },
        CubeSet { color: "green", count: 13 },
        CubeSet { color: "blue", count: 14 },
    ];
    bag
        .into_iter()
        .any(|x| x.color == cubeset.color && cubeset.count <= x.count)
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

    // Available cubes in bag defined in prompt

    rounds.fold(0, |acc, round| {
        let game_is_possible = round.cube_groups
            .into_iter()
            .all(|cube_group| {
                cube_group.into_iter().all(|cubeset| check_cubeset(cubeset))
            });
        
        if game_is_possible { acc + round.id } else { acc }
    }).to_string()
}
