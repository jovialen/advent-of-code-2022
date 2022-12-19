#[derive(Debug)]
struct OreRobot(u32);
#[derive(Debug)]
struct ClayRobot(u32);
#[derive(Debug)]
struct ObsidianRobot(u32, u32);
#[derive(Debug)]
struct GeodeRobot(u32, u32);

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot: OreRobot,
    clay_robot: ClayRobot,
    obsidian_robot: ObsidianRobot,
    geode_robot: GeodeRobot,
}

impl From<&str> for Blueprint {
    fn from(src: &str) -> Self {
        let mut iter = src
            .split(&[' ', ':'])
            .filter_map(|word| word.parse::<u32>().ok());

        Self {
            id: iter.next().unwrap_or(0),
            ore_robot: OreRobot(iter.next().unwrap_or(0)),
            clay_robot: ClayRobot(iter.next().unwrap_or(0)),
            obsidian_robot: ObsidianRobot(iter.next().unwrap_or(0), iter.next().unwrap_or(0)),
            geode_robot: GeodeRobot(iter.next().unwrap_or(0), iter.next().unwrap_or(0)),
        }
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Blueprint::from)
        .collect()
}

fn main() {
    let blueprints = parse_input(include_str!("../input.txt"));
    for blueprint in blueprints {
        println!("{:?}", blueprint);
    }
}
