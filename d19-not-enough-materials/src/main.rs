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

impl TryFrom<&str> for Blueprint {
    type Error = ();

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut iter = src
            .split(&[' ', ':'])
            .filter_map(|word| word.parse::<u32>().ok());

        Ok(Self {
            id: iter.next().ok_or(())?,
            ore_robot: OreRobot(iter.next().ok_or(())?),
            clay_robot: ClayRobot(iter.next().ok_or(())?),
            obsidian_robot: ObsidianRobot(iter.next().ok_or(())?, iter.next().ok_or(())?),
            geode_robot: GeodeRobot(iter.next().ok_or(())?, iter.next().ok_or(())?),
        })
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .filter_map(|line| Blueprint::try_from(line).ok())
        .collect()
}

fn main() {
    let blueprints = parse_input(include_str!("../input.txt"));
    for blueprint in blueprints {
        println!("{:?}", blueprint);
    }
}
